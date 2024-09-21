mod menus;

use super::utils::window::{hide_window, show_window};
use super::about;
use menus::{ABOUT_MENU, HIDE_MENU, QUIT_MENU, SETTINGS_MENU, SHOW_MENU};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use crate::screens::{MAIN, SETTINGS};

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
                if let Err(err) = show_window(&app.get_window(about::SCREEN)) {
                    log::error!("Unable to show about window from tray. {err}");
                }
            }
            HIDE_MENU => {
                if let Err(err) = hide_window(&app.get_window(MAIN)) {
                    log::error!("Unable to hide main window from tray. {err}");
                }
            }
            SHOW_MENU => {
                if let Err(err) = show_window(&app.get_window(MAIN)) {
                    log::error!("Unable to show main window from tray. {err}");
                }
            }
            SETTINGS_MENU => {
                if let Err(err) = show_window(&app.get_window(SETTINGS)) {
                    log::error!("Unable to show settings window from tray. {err}");
                }
            }
            _ => {}
        },
        _ => {}
    }
}
