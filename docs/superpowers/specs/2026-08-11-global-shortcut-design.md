# Global Shortcut Design: Super+Shift+V & Ctrl+Shift+V Support

## Overview
ClipKeeper currently supports `Ctrl+Shift+V` to toggle the clipboard palette. This design adds support for `Super+Shift+V` (Super/Win/Meta + Shift + V), standardizing global clipboard access on Linux desktop environments (GNOME/KDE/XFCE) while maintaining `Ctrl+Shift+V` for backwards compatibility and user convenience.

## Changes Required

### 1. Tauri Backend (`src-tauri/src/lib.rs`)
- Define two `Shortcut` instances:
  - `super_shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyV)`
  - `ctrl_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV)`
- Update global shortcut event handler:
  - Match either `shortcut == &super_shortcut` or `shortcut == &ctrl_shortcut` when `ShortcutState::Pressed`.
  - Toggle window visibility (show/set_focus or hide).
- Update `.setup()` block:
  - Register both `super_shortcut` and `ctrl_shortcut` with `app.global_shortcut().register()`.

### 2. Architecture Spec (`AI_AGENTS.md`)
- Update `AI_AGENTS.md` architecture overview to list `global_shortcut: Super+Shift+V and Ctrl+Shift+V`.

## Verification Plan
1. Run `cargo check` in `src-tauri` directory to ensure clean compilation.
2. Build/test the application with `npm run tauri dev` or `cargo check` to verify global shortcut registration.
