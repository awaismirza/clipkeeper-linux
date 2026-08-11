# Auto-Paste on Enter Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement auto-paste on `Enter` keypress (and click) in ClipKeeper, using `Shift+Enter` as copy-only fallback, powered by `enigo` Rust keyboard simulation.

**Architecture:** Update `copy_to_clipboard` IPC command in `commands.rs` to accept an optional `paste: Option<bool>` parameter. When `paste` is true, after updating system clipboard and hiding the Tauri window, spawn a background thread that waits 80ms for OS focus transition and simulates `Ctrl+V`. Update `main.js` keydown and click event handlers.

**Tech Stack:** Rust, Tauri v2, `enigo` crate (`0.3`), Vanilla JavaScript (ES6+).

## Global Constraints
- Target platform: Linux (Ubuntu / Wayland / X11).
- Cargo build flag for SQLite: `bundled`.
- Non-blocking UX: Window hides immediately; keypress simulation runs in a background thread.

---

### Task 1: Backend Keyboard Simulation & IPC Command Update

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/commands.rs:30-81`

**Interfaces:**
- Consumes: Tauri Window, State, `arboard::Clipboard`, `enigo::{Enigo, Keyboard, Key, Direction}`
- Produces: Updated `copy_to_clipboard(id: i64, paste: Option<bool>, window: Window, state: State<'_, Mutex<AppState>>) -> Result<(), String>` IPC command handler.

- [ ] **Step 1: Add `enigo` dependency to `src-tauri/Cargo.toml`**

Add `enigo = "0.3"` under `[dependencies]` in `src-tauri/Cargo.toml`.

- [ ] **Step 2: Update `copy_to_clipboard` IPC command in `src-tauri/src/commands.rs`**

Update `copy_to_clipboard` signature to accept `paste: Option<bool>`:
```rust
#[tauri::command]
pub fn copy_to_clipboard(
    id: i64,
    paste: Option<bool>,
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
        clipboard
            .set_text(&item.content)
            .map_err(|e| format!("Set clipboard text failed: {}", e))?;
    }

    let should_paste = paste.unwrap_or(true);

    // Hide window after copy action
    let _ = window.hide();

    if should_paste {
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(80));
            if let Ok(mut enigo) = enigo::Enigo::new(&enigo::Settings::default()) {
                use enigo::Keyboard;
                let _ = enigo.key(enigo::Key::Control, enigo::Direction::Press);
                let _ = enigo.key(enigo::Key::Unicode('v'), enigo::Direction::Click);
                let _ = enigo.key(enigo::Key::Control, enigo::Direction::Release);
            }
        });
    }

    Ok(())
}
```

- [ ] **Step 3: Run `cargo check` in `src-tauri` to verify backend build**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo check` in `src-tauri` directory.
Expected: Finished successfully.

- [ ] **Step 4: Commit Task 1 changes**

```bash
git add src-tauri/Cargo.toml src-tauri/src/commands.rs
git commit -m "feat(backend): add enigo dependency and auto-paste support to copy_to_clipboard"
```

---

### Task 2: Frontend Keybindings & Click Event Updates

**Files:**
- Modify: `src/main.js:161-175,204-213,297-303`

**Interfaces:**
- Consumes: Tauri IPC `copy_to_clipboard` with `{ id, paste }`.
- Produces: Updated `selectAndCopyItem(id, paste = true)` helper and updated event handlers.

- [ ] **Step 1: Update `selectAndCopyItem` in `src/main.js`**

```javascript
async function selectAndCopyItem(id, paste = true) {
  if (!invoke) return;

  try {
    await invoke('copy_to_clipboard', { id, paste });
  } catch (err) {
    console.error('Failed to copy to clipboard:', err);
  }
}
```

- [ ] **Step 2: Update click event listener on item card in `renderList()`**

```javascript
    card.addEventListener('click', (e) => {
      const action = e.target.closest('[data-action]')?.dataset.action;
      if (action === 'pin') {
        e.stopPropagation();
        togglePinItem(item.id);
      } else if (action === 'delete') {
        e.stopPropagation();
        deleteItem(item.id);
      } else {
        selectedIndex = index;
        updateSelectionHighlight();
        const shouldPaste = !e.shiftKey;
        selectAndCopyItem(item.id, shouldPaste);
      }
    });
```

- [ ] **Step 3: Update `Enter` keydown handler in `setupEventListeners()`**

```javascript
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const currentItem = historyItems[selectedIndex];
      if (currentItem) {
        const shouldPaste = !e.shiftKey;
        selectAndCopyItem(currentItem.id, shouldPaste);
      }
    }
```

- [ ] **Step 4: Commit Task 2 changes**

```bash
git add src/main.js
git commit -m "feat(frontend): add Enter (auto-paste) and Shift+Enter (copy-only) keyboard & click handlers"
```

---

### Task 3: Build Verification, Commit & Release

**Files:**
- Modify: `docs/superpowers/plans/2026-08-12-auto-paste-on-enter.md`

- [ ] **Step 1: Run Cargo Check & Build verification**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo check` in `src-tauri`.
Run: `npm run build` or `npm run tauri build` to verify release build.

- [ ] **Step 2: Commit all remaining work & merge branch**

Merge `feature/auto-paste-on-enter` into `main`.
