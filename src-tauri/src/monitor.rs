use crate::storage::Database;
use arboard::{Clipboard, ImageData};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::{DynamicImage, GenericImageView, ImageEncoder};
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

pub fn start_clipboard_monitor(app_handle: AppHandle, db: Database) {
    let running = Arc::new(AtomicBool::new(true));

    thread::spawn(move || {
        let mut last_hash = String::new();

        // Give clipboard system time to initialize
        thread::sleep(Duration::from_millis(500));

        while running.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(350));

            let mut clipboard = match Clipboard::new() {
                Ok(cb) => cb,
                Err(_) => continue,
            };

            // 1. Try reading Text
            if let Ok(text) = clipboard.get_text() {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    let mut hasher = Sha256::new();
                    hasher.update(trimmed.as_bytes());
                    let hash = format!("{:x}", hasher.finalize());

                    if hash != last_hash {
                        last_hash = hash.clone();

                        let item_type = detect_text_type(trimmed);
                        let preview = create_text_preview(trimmed, 180);

                        if let Ok(new_id) = db.insert_or_update(
                            &item_type,
                            trimmed,
                            &preview,
                            None,
                            None,
                            &hash,
                        ) {
                            let _ = app_handle.emit("clipboard-updated", serde_json::json!({ "id": new_id, "type": item_type }));
                        }
                    }
                    continue;
                }
            }

            // 2. Try reading Image
            if let Ok(image_data) = clipboard.get_image() {
                if let Some((png_data_url, width, height, hash)) = process_image_data(&image_data) {
                    if hash != last_hash {
                        last_hash = hash.clone();

                        if let Ok(new_id) = db.insert_or_update(
                            "image",
                            &png_data_url,
                            &png_data_url, // For image, preview is thumbnail data URL
                            Some(width),
                            Some(height),
                            &hash,
                        ) {
                            let _ = app_handle.emit("clipboard-updated", serde_json::json!({ "id": new_id, "type": "image" }));
                        }
                    }
                }
            }
        }
    });
}

fn detect_text_type(text: &str) -> String {
    let lower = text.to_lowercase();
    if lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("www.")
        || (lower.contains("://") && !text.contains('\n'))
    {
        return "url".to_string();
    }

    let is_multiline = text.contains('\n');
    let code_indicators = [
        "fn ", "function", "const ", "let ", "var ", "pub ", "struct ", "impl ",
        "class ", "import ", "export ", "def ", "return ", "if (", "if ", "else {",
        "SELECT ", "FROM ", "WHERE ", "<div", "<script", "```", ";\n", "{\n",
    ];

    let has_code_token = code_indicators.iter().any(|&token| text.contains(token));

    if has_code_token {
        "code".to_string()
    } else if is_multiline {
        "multiline".to_string()
    } else {
        "text".to_string()
    }
}

fn create_text_preview(text: &str, max_len: usize) -> String {
    let cleaned = text.lines().collect::<Vec<_>>().join(" ");
    if cleaned.chars().count() > max_len {
        format!("{}...", cleaned.chars().take(max_len).collect::<String>())
    } else {
        cleaned
    }
}

fn process_image_data(image_data: &ImageData) -> Option<(String, u32, u32, String)> {
    let width = image_data.width as u32;
    let height = image_data.height as u32;

    if width == 0 || height == 0 {
        return None;
    }

    let img_buffer = image::RgbaImage::from_raw(width, height, image_data.bytes.to_vec())?;
    let dyn_img = DynamicImage::ImageRgba8(img_buffer);

    // Downscale huge images for memory/storage efficiency
    let resized = if width > 1024 || height > 1024 {
        dyn_img.thumbnail(1024, 1024)
    } else {
        dyn_img
    };

    let (res_w, res_h) = resized.dimensions();

    let mut png_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut png_bytes);

    let encoder = image::codecs::png::PngEncoder::new(&mut cursor);
    if encoder
        .write_image(
            resized.to_rgba8().as_raw(),
            res_w,
            res_h,
            image::ExtendedColorType::Rgba8,
        )
        .is_err()
    {
        return None;
    }

    let mut hasher = Sha256::new();
    hasher.update(&png_bytes);
    let hash = format!("{:x}", hasher.finalize());

    let b64 = BASE64.encode(&png_bytes);
    let data_url = format!("data:image/png;base64,{}", b64);

    Some((data_url, width, height, hash))
}
