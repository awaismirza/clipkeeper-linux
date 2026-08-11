use crate::models::ClipboardItem;
use rusqlite::{params, Connection, Result as SqlResult};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(db_path: PathBuf) -> SqlResult<Self> {
        if let Some(parent) = db_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let conn = Connection::open(db_path)?;

        // Enable PRAGMAs for speed & safety
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS clipboard_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hash TEXT UNIQUE NOT NULL,
                item_type TEXT NOT NULL,
                content TEXT NOT NULL,
                preview TEXT NOT NULL,
                image_width INTEGER,
                image_height INTEGER,
                timestamp INTEGER NOT NULL,
                pinned INTEGER NOT NULL DEFAULT 0
            );",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON clipboard_history(timestamp DESC);",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_item_type ON clipboard_history(item_type);",
            [],
        )?;

        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn insert_or_update(
        &self,
        item_type: &str,
        content: &str,
        preview: &str,
        image_width: Option<u32>,
        image_height: Option<u32>,
        hash: &str,
    ) -> SqlResult<i64> {
        let conn = self.conn.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        // Check if hash exists; if so, update timestamp to bring it to top
        let mut stmt = conn.prepare("SELECT id FROM clipboard_history WHERE hash = ?1")?;
        let existing: Option<i64> = stmt.query_row(params![hash], |row| row.get(0)).ok();

        if let Some(existing_id) = existing {
            conn.execute(
                "UPDATE clipboard_history SET timestamp = ?1 WHERE id = ?2",
                params![now, existing_id],
            )?;
            return Ok(existing_id);
        }

        // Insert new entry
        conn.execute(
            "INSERT INTO clipboard_history (hash, item_type, content, preview, image_width, image_height, timestamp, pinned)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0)",
            params![hash, item_type, content, preview, image_width, image_height, now],
        )?;

        let new_id = conn.last_insert_rowid();

        // Enforce 500 max item limit (trim oldest unpinned entries)
        let _ = conn.execute(
            "DELETE FROM clipboard_history 
             WHERE pinned = 0 AND id NOT IN (
                SELECT id FROM clipboard_history 
                ORDER BY timestamp DESC 
                LIMIT 500
             );",
            [],
        );

        Ok(new_id)
    }

    pub fn get_items(
        &self,
        search: Option<String>,
        filter_type: Option<String>,
        limit: usize,
    ) -> SqlResult<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();
        let limit_val = limit.min(500) as i64;

        let mut query = String::from(
            "SELECT id, hash, item_type, content, preview, image_width, image_height, timestamp, pinned 
             FROM clipboard_history WHERE 1=1 ",
        );

        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref ftype) = filter_type {
            if ftype != "all" && !ftype.is_empty() {
                if ftype == "pinned" {
                    query.push_str(" AND pinned = 1 ");
                } else {
                    query.push_str(" AND item_type = ? ");
                    params_vec.push(Box::new(ftype.clone()));
                }
            }
        }

        if let Some(ref q) = search {
            if !q.trim().is_empty() {
                query.push_str(" AND (preview LIKE ? OR content LIKE ?) ");
                let search_pattern = format!("%{}%", q.trim());
                params_vec.push(Box::new(search_pattern.clone()));
                params_vec.push(Box::new(search_pattern));
            }
        }

        query.push_str(" ORDER BY pinned DESC, timestamp DESC LIMIT ?");
        params_vec.push(Box::new(limit_val));

        let mut stmt = conn.prepare(&query)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

        let rows = stmt.query_map(params_refs.as_slice(), |row| {
            let pinned_int: i32 = row.get(8)?;
            Ok(ClipboardItem {
                id: row.get(0)?,
                hash: row.get(1)?,
                item_type: row.get(2)?,
                content: row.get(3)?,
                preview: row.get(4)?,
                image_width: row.get(5)?,
                image_height: row.get(6)?,
                timestamp: row.get(7)?,
                pinned: pinned_int != 0,
            })
        })?;

        let mut items = Vec::new();
        for item in rows {
            items.push(item?);
        }

        Ok(items)
    }

    pub fn get_by_id(&self, id: i64) -> SqlResult<Option<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, hash, item_type, content, preview, image_width, image_height, timestamp, pinned 
             FROM clipboard_history WHERE id = ?1",
        )?;

        let mut rows = stmt.query_map(params![id], |row| {
            let pinned_int: i32 = row.get(8)?;
            Ok(ClipboardItem {
                id: row.get(0)?,
                hash: row.get(1)?,
                item_type: row.get(2)?,
                content: row.get(3)?,
                preview: row.get(4)?,
                image_width: row.get(5)?,
                image_height: row.get(6)?,
                timestamp: row.get(7)?,
                pinned: pinned_int != 0,
            })
        })?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    pub fn delete_item(&self, id: i64) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM clipboard_history WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn clear_all(&self) -> SqlResult<()> {
        let conn = self.conn.lock().unwrap();
        // Clear unpinned items only to protect pinned content
        conn.execute("DELETE FROM clipboard_history WHERE pinned = 0", [])?;
        Ok(())
    }

    pub fn toggle_pin(&self, id: i64) -> SqlResult<bool> {
        let conn = self.conn.lock().unwrap();
        let current_pinned: i32 = conn
            .query_row(
                "SELECT pinned FROM clipboard_history WHERE id = ?1",
                params![id],
                |r| r.get(0),
            )
            .unwrap_or(0);

        let new_pinned = if current_pinned == 0 { 1 } else { 0 };
        conn.execute(
            "UPDATE clipboard_history SET pinned = ?1 WHERE id = ?2",
            params![new_pinned, id],
        )?;

        Ok(new_pinned == 1)
    }
}
