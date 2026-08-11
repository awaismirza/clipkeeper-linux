# Design Specification: Auto-Paste on Enter Feature

## Overview
ClipKeeper currently updates the system clipboard when an item is selected and copied, but requires the user to manually press `Ctrl+V` in their target application. This feature adds auto-paste functionality: pressing `Enter` on a clipboard item will copy it and automatically paste it (`Ctrl+V`) into the last focused window/input area. Pressing `Shift+Enter` will copy the item without auto-pasting.

## User Interaction Model
- **`Enter`**: Copies item to clipboard $\rightarrow$ hides ClipKeeper window $\rightarrow$ auto-pastes (`Ctrl+V`) into active window.
- **`Shift+Enter`**: Copies item to clipboard $\rightarrow$ hides ClipKeeper window (copy only, no auto-paste).
- **Mouse Click**: Clicking an item auto-pastes by default; holding `Shift` while clicking copies without auto-pasting.

## Architecture & Implementation Plan

### 1. Dependencies (`src-tauri/Cargo.toml`)
- Add `enigo = "0.3"` (or latest compatible version) to `src-tauri/Cargo.toml` for simulating OS keyboard input on Linux.

### 2. Backend IPC Update (`src-tauri/src/commands.rs`)
- Update `copy_to_clipboard` handler signature:
  ```rust
  pub fn copy_to_clipboard(
      id: i64,
      paste: Option<bool>,
      window: Window,
      state: State<'_, Mutex<AppState>>,
  ) -> Result<(), String>
  ```
- Implementation details:
  1. Fetch clipboard item from SQLite database by `id`.
  2. Set item text or PNG image data on system clipboard (`arboard`).
  3. Hide palette window (`window.hide()`).
  4. If `paste` is `Some(true)` (defaulting to `true` when omitted for backward compatibility):
     - Spawn a thread (`std::thread::spawn`) that sleeps ~80ms to permit OS window manager focus restoration.
     - Simulate pressing `Ctrl+V` using `Enigo`.

### 3. Frontend Controller Update (`src/main.js`)
- In `setupEventListeners()` keyboard handler:
  - When `e.key === 'Enter'`:
    - Determine `shouldPaste = !e.shiftKey`.
    - Call `selectAndCopyItem(currentItem.id, shouldPaste)`.
- In `selectAndCopyItem(id, paste = true)`:
  - Invoke `copy_to_clipboard` IPC command passing `{ id, paste }`.
- In `historyList` click handler:
  - Pass `!e.shiftKey` as the `paste` argument when item is clicked.

## Verification Plan
1. Run `PATH="$HOME/.cargo/bin:$PATH" cargo check` in `src-tauri` to verify Rust code compilation.
2. Build binary or run `npm run tauri dev` to test `Enter` and `Shift+Enter` behavior in test applications.
