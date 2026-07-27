#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn main() {
    let show_hide = CustomMenuItem::new("show_hide".to_string(), "Show/Hide ApplyBee");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit TaskPets");
    let tray_menu = SystemTrayMenu::new().add_item(show_hide).add_item(quit);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "show_hide" => {
                        if let Some(window) = app.get_window("applybee") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    "quit" => app.exit(0),
                    _ => {}
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run TaskPets desktop app");
}
