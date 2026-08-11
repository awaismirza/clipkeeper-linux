use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: i64,
    pub hash: String,
    pub item_type: String, // "text", "url", "code", "multiline", "image"
    pub content: String,   // Raw text or base64 image data URL
    pub preview: String,   // Truncated preview snippet or base64 thumbnail
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub timestamp: i64,    // Unix timestamp (ms)
    pub pinned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterQuery {
    pub search: Option<String>,
    pub filter_type: Option<String>,
    pub limit: Option<usize>,
}
