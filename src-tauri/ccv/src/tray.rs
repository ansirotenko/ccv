use crate::commands::{
    about::show_about_window,
    main::{hide_main_window, show_main_window},
    settings::show_settings_window,
};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

const ABOUT_MENU: &str = "about";
const SETTINGS_MENU: &str = "settings";
const SHOW_MENU: &str = "show";
const HIDE_MENU: &str = "hide";
const QUIT_MENU: &str = "quit";

pub fn get_tray_menu() -> SystemTray {
    let about = CustomMenuItem::new(ABOUT_MENU.to_string(), "About");
    let settings = CustomMenuItem::new(SETTINGS_MENU.to_string(), "Settings");
    let show = CustomMenuItem::new(SHOW_MENU.to_string(), "Show");
    let hide = CustomMenuItem::new(HIDE_MENU.to_string(), "Hide");
    let quit = CustomMenuItem::new(QUIT_MENU.to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(about)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn tray_event_handler(app: &tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            QUIT_MENU => {
                std::process::exit(0);
            }
            ABOUT_MENU => {
                show_about_window(app.clone()).unwrap();
            }
            HIDE_MENU => {
                hide_main_window(app.clone()).unwrap();
            }
            SHOW_MENU => {
                show_main_window(app.clone()).unwrap();
            }
            SETTINGS_MENU => {
                show_settings_window(app.clone()).unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
