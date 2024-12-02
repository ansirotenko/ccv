// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use tauri::{async_runtime, generate_context, Builder, Manager};

fn main() {
    Builder::default()
        .setup(|app| {
            let app_handle_clone1 = app.app_handle().clone();
            let app_handle_clone2 = app.app_handle().clone();
            async_runtime::spawn(async move {
                loop {
                    thread::sleep(std::time::Duration::from_millis(100));
                    if let Some(window) = app_handle_clone1.get_webview_window("primary") {
                        println!("{}", window.is_focused().unwrap());
                    }
                }
            });
            async_runtime::spawn(async move {
                loop {
                    thread::sleep(std::time::Duration::from_millis(5_000));
                    if let Some(window) = app_handle_clone2.get_webview_window("primary") {
                        println!("Here");
                        window.set_focus().unwrap();
                        window.show().unwrap();
                    }
                }
            });
            Ok(())
        })
        .run(generate_context!())
        .unwrap()
        ;
}
