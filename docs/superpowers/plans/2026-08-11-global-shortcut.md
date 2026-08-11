# Super+Shift+V Global Shortcut Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Register `Super+Shift+V` alongside existing `Ctrl+Shift+V` in Tauri v2 global shortcut plugin so either shortcut toggles the ClipKeeper window.

**Architecture:** Update `src-tauri/src/lib.rs` to initialize and register both `super_shortcut` (`Modifiers::SUPER | Modifiers::SHIFT + KeyV`) and `ctrl_shortcut` (`Modifiers::CONTROL | Modifiers::SHIFT + KeyV`), and update `AI_AGENTS.md` docs.

**Tech Stack:** Rust (Tauri v2, tauri-plugin-global-shortcut)

## Global Constraints

- Must compile cleanly with `PATH="$HOME/.cargo/bin:$PATH" cargo check` in `src-tauri` directory.
- Preserve existing `Ctrl+Shift+V` functionality while adding `Super+Shift+V`.

---

### Task 1: Update Global Shortcut Registration in Tauri Backend

**Files:**
- Modify: `src-tauri/src/lib.rs:24-68`
- Modify: `AI_AGENTS.md:26`

**Interfaces:**
- Consumes: `tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState}`
- Produces: Global shortcut registrations for `Super+Shift+V` and `Ctrl+Shift+V`.

- [ ] **Step 1: Update shortcut registration and handler in lib.rs**

In `src-tauri/src/lib.rs`:
Update the `tauri_plugin_global_shortcut::Builder::new().with_handler(...)` closure to check for both `super_shortcut` (`Modifiers::SUPER | Modifiers::SHIFT`, `Code::KeyV`) and `ctrl_shortcut` (`Modifiers::CONTROL | Modifiers::SHIFT`, `Code::KeyV`).
In `.setup(...)`, call `app.global_shortcut().register(super_shortcut)` and `app.global_shortcut().register(ctrl_shortcut)`.

- [ ] **Step 2: Verify Rust build compilation**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo check` in `src-tauri` directory.
Expected: PASS with 0 errors.

- [ ] **Step 3: Update AI_AGENTS.md architecture documentation**

Update `AI_AGENTS.md` line 26 to reflect `global_shortcut: tauri-plugin-global-shortcut (Super+Shift+V, Ctrl+Shift+V)`.
