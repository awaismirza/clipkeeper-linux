mod commands;
mod models;
mod monitor;
mod storage;

use commands::*;
use monitor::start_clipboard_monitor;
use storage::Database;
use std::sync::Mutex;
use tauri::{Manager, WindowEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            eprintln!("[DEBUG] Single instance invoked with args: {:?}", args);
            if let Some(window) = app.get_webview_window("main") {
                if args.iter().any(|a| a == "--toggle" || a == "toggle") {
                    let is_vis = window.is_visible().unwrap_or(false);
                    eprintln!("[DEBUG] Single instance toggle requested. Visibility: {}", is_vis);
                    if is_vis {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                    }
                } else {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
        }))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    eprintln!("[DEBUG] Shortcut event received: {:?}, state: {:?}", shortcut, event.state());
                    if event.state() == ShortcutState::Pressed {
                        let alt_shortcut = Shortcut::new(
                            Some(Modifiers::ALT | Modifiers::SHIFT),
                            Code::KeyV,
                        );

                        let shortcut_str = shortcut.to_string().to_lowercase();
                        eprintln!("[DEBUG] Shortcut string: {}", shortcut_str);

                        let is_match = shortcut == &alt_shortcut
                            || (shortcut_str.contains("alt") && shortcut_str.contains("v"));

                        if is_match {
                            eprintln!("[DEBUG] Shortcut matched! Toggling window...");
                            if let Some(window) = app.get_webview_window("main") {
                                let is_vis = window.is_visible().unwrap_or(false);
                                eprintln!("[DEBUG] Current window visibility: {}", is_vis);
                                if is_vis {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.unminimize();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Locate app data directory for SQLite storage
            let app_data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."));

            let db_path = app_data_dir.join("clipkeeper.db");
            let db = Database::new(db_path).expect("Failed to initialize ClipKeeper database");

            // Store state
            let db_for_state = db.clone();
            app.manage(Mutex::new(AppState { db: db_for_state }));

            // Register global shortcut: Alt+Shift+V
            let alt_shortcut = Shortcut::new(
                Some(Modifiers::ALT | Modifiers::SHIFT),
                Code::KeyV,
            );

            match app.global_shortcut().register(alt_shortcut) {
                Ok(_) => eprintln!("[DEBUG] Successfully registered Alt+Shift+V ({:?})", alt_shortcut),
                Err(e) => eprintln!("[ERROR] Failed to register Alt+Shift+V: {:?}", e),
            }

            // Add system tray icon for top bar in Ubuntu
            if let Some(icon) = app.default_window_icon() {
                use tauri::menu::{MenuBuilder, MenuItemBuilder};
                use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

                let toggle_item = MenuItemBuilder::with_id("toggle", "Show/Hide ClipKeeper").build(app)?;
                let quit_item = MenuItemBuilder::with_id("quit", "Quit ClipKeeper").build(app)?;
                let tray_menu = MenuBuilder::new(app)
                    .item(&toggle_item)
                    .separator()
                    .item(&quit_item)
                    .build()?;

                let _tray = TrayIconBuilder::new()
                    .icon(icon.clone())
                    .tooltip("ClipKeeper Clipboard Manager (Alt+Shift+V)")
                    .menu(&tray_menu)
                    .show_menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "toggle" => {
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.unminimize();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.unminimize();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    })
                    .build(app)?;
            }

            // Start background clipboard polling worker thread
            start_clipboard_monitor(app_handle, db);

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::Focused(focused) = event {
                eprintln!("[DEBUG] Window focused event: {}", focused);
                if !focused {
                    // Hide window when focus is lost (Palette UX behavior)
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_history,
            copy_to_clipboard,
            delete_item,
            clear_history,
            toggle_pin,
            hide_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running ClipKeeper application");
}
