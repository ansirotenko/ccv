// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod screens;
mod commands;
mod tray;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .setup(|app| {
            let splashscreen_window = app.get_window(screens::SPLASHSCREEN).unwrap();
            let main_window = app.get_window(screens::MAIN).unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // TODO initialize app here instead of sleeping :)
                println!("Initializing...");
                std::thread::sleep(std::time::Duration::from_secs(1));
                println!("Done initializing.");

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });

            let handle = app.handle();
            let clipboard = handle.state::<tauri_plugin_clipboard::ClipboardManager>();
            clipboard.write_text("huakun zui shuai".to_string()).unwrap();
        
            Ok(())
        })
        .system_tray(tray::get_tray_menu())
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_system_tray_event(tray::tray_event_handler)
        .invoke_handler(tauri::generate_handler![
            commands::search_copy_items,
            commands::hide_window,
            commands::show_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
