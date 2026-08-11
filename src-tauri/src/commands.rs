use crate::models::ClipboardItem;
use crate::storage::Database;
use arboard::{Clipboard, ImageData};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::GenericImageView;
use std::borrow::Cow;
use std::sync::Mutex;
use tauri::{State, Window};

pub struct AppState {
    pub db: Database,
}

#[tauri::command]
pub fn get_history(
    search: Option<String>,
    filter_type: Option<String>,
    limit: Option<usize>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<ClipboardItem>, String> {
    let guard = state.lock().map_err(|e| e.to_string())?;
    let limit_val = limit.unwrap_or(200);
    guard
        .db
        .get_items(search, filter_type, limit_val)
        .map_err(|e| format!("Database query error: {}", e))
}

#[tauri::command]
pub fn copy_to_clipboard(
    id: i64,
    window: Window,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let guard = state.lock().map_err(|e| e.to_string())?;
    let item = guard
        .db
        .get_by_id(id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Clipboard item not found".to_string())?;

    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard error: {}", e))?;

    if item.item_type == "image" {
        // Restore image to clipboard
        let base64_str = item
            .content
            .strip_prefix("data:image/png;base64,")
            .unwrap_or(&item.content);

        let img_bytes = BASE64
            .decode(base64_str)
            .map_err(|e| format!("Base64 decode error: {}", e))?;

        let img = image::load_from_memory(&img_bytes)
            .map_err(|e| format!("Image load error: {}", e))?;

        let (width, height) = img.dimensions();
        let rgba = img.to_rgba8();

        let image_data = ImageData {
            width: width as usize,
            height: height as usize,
            bytes: Cow::Owned(rgba.into_raw()),
        };

        clipboard
            .set_image(image_data)
            .map_err(|e| format!("Set clipboard image failed: {}", e))?;
    } else {
        // Restore text to clipboard
        clipboard
            .set_text(&item.content)
            .map_err(|e| format!("Set clipboard text failed: {}", e))?;
    }

    // Hide window after copy/paste action
    let _ = window.hide();

    Ok(())
}

#[tauri::command]
pub fn delete_item(id: i64, state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let guard = state.lock().map_err(|e| e.to_string())?;
    guard.db.delete_item(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_history(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let guard = state.lock().map_err(|e| e.to_string())?;
    guard.db.clear_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_pin(id: i64, state: State<'_, Mutex<AppState>>) -> Result<bool, String> {
    let guard = state.lock().map_err(|e| e.to_string())?;
    guard.db.toggle_pin(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn hide_window(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}
