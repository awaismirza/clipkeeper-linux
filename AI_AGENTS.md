# AI AGENTS & ARCHITECTURE SPECIFICATION: ClipKeeper

## Project Overview
**ClipKeeper** is an ultra-lightweight, production-ready system clipboard manager built with **Tauri v2** and **Rust**, specifically optimized for **Linux ARM64** (Ubuntu inside Parallels VM / bare-metal ARM) supporting both X11 and Wayland environments.

---

## 1. Subsystem Architecture & Responsibilities

```
+-------------------------------------------------------------------+
|                        ClipKeeper Architecture                    |
+-------------------------------------------------------------------+
|  [ Frontend UI ]                                                  |
|  - Vanilla HTML/JS + Tailwind CSS floating search palette         |
|  - Real-time fuzzy filtering & type tabs (All, Text, Code, Image) |
|  - Keyboard navigation (Arrow keys, Enter, Esc, Ctrl+D, P)        |
+-------------------------------------------------------------------+
                               |  Tauri IPC (invoke)
                               v
+-------------------------------------------------------------------+
|  [ Tauri v2 Backend (Rust) ]                                      |
|  - lib.rs / commands.rs: IPC API handlers                         |
|  - storage.rs: SQLite storage (rusqlite) capped at 500 items       |
|  - monitor.rs: Low-CPU clipboard polling thread (arboard)          |
|  - global_shortcut: tauri-plugin-global-shortcut (Super+Shift+V, Ctrl+Shift+V) |
|  - single_instance: tauri-plugin-single-instance                  |
+-------------------------------------------------------------------+
```

---

## 2. Directory Layout & File Responsibilities

- **`AI_AGENTS.md`**: Architectural reference and guidelines for AI coding agents.
- **`src-tauri/Cargo.toml`**: Cargo dependencies including `tauri`, `rusqlite` (bundled), `arboard`, `image`, `base64`, `sha2`, `tauri-plugin-global-shortcut`, `tauri-plugin-single-instance`.
- **`src-tauri/tauri.conf.json`**: Window configuration (frameless, transparent,Palette sizing, always-on-top, skip taskbar) and plugin registrations.
- **`src-tauri/capabilities/default.json`**: Permission capabilities for Tauri v2 global shortcuts and window controls.
- **`src-tauri/src/lib.rs`**: Main Tauri builder entrypoint, plugin setup, system tray/hotkey registration, and state initialization.
- **`src-tauri/src/storage.rs`**: SQLite data layer (`rusqlite` with bundled SQLite engine). Tables for clipboard items, index on timestamp, capped at 500 items automatically.
- **`src-tauri/src/monitor.rs`**: Low-CPU background thread that polls `arboard::Clipboard` for text, HTML, and rich image clips without high CPU usage.
- **`src-tauri/src/commands.rs`**: Tauri IPC command handlers exposed to frontend.
- **`src/index.html`**: Main floating palette UI shell.
- **`src/styles.css`**: Design system tokens, dark mode styles, custom scrollbars, palette layout.
- **`src/main.js`**: Frontend controller, keyboard event handlers, IPC communication bridge, image thumbnail rendering.

---

## 3. Developer & AI Agent Working Guidelines

### 3.1 Strict Rules
1. **Linux ARM Compatibility:** Always use `rusqlite` with feature `"bundled"` so compiling SQLite does not rely on host system C libraries.
2. **Resource Efficiency:** Background polling loop in `monitor.rs` MUST sleep between checks (~350ms) and use SHA-256 content hashing to eliminate redundant DB writes and CPU spikes.
3. **Graceful Degradation:** Clipboard read operations must handle empty or unsupported data gracefully without panicking.
4. **Clean Exit & Single Instance:** Ensure only one instance of ClipKeeper runs at a time using `tauri-plugin-single-instance`.

### 3.2 Key Commands
- Check Rust code: `PATH="$HOME/.cargo/bin:$PATH" cargo check` (in `src-tauri`)
- Dev server: `npm run tauri dev`
- Build release: `npm run tauri build`

---

## 4. AI Subagent Invocation Protocol
- **Codebase Researcher:** Use `research` subagent to explore files or search docs when needed.
- **Verification:** Always run `cargo check` and verify file modifications before marking tasks completed.
