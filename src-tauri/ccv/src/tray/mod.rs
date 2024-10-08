mod menus;

use crate::about;
use crate::primary;
use crate::settings;
use crate::utils::window::show_window;
use menus::{ABOUT_MENU, QUIT_MENU, SETTINGS_MENU, SHOW_MENU};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub fn get_menu() -> SystemTray {
    let about = CustomMenuItem::new(ABOUT_MENU.to_string(), "About");
    let settings = CustomMenuItem::new(SETTINGS_MENU.to_string(), "Settings");
    let show = CustomMenuItem::new(SHOW_MENU.to_string(), "Show");
    let quit = CustomMenuItem::new(QUIT_MENU.to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(about)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn event_handler(app_handle: &tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {}
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {}
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            if let Err(err) = show_window(&app_handle.get_window(primary::SCREEN)) {
                log::error!("Unable to show primary window from tray. {err}");
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            QUIT_MENU => {
                std::process::exit(0);
            }
            ABOUT_MENU => {
                if let Err(err) = show_window(&app_handle.get_window(about::SCREEN)) {
                    log::error!("Unable to show about window from tray. {err}");
                }
            }
            SHOW_MENU => {
                if let Err(err) = show_window(&app_handle.get_window(primary::SCREEN)) {
                    log::error!("Unable to show primary window from tray. {err}");
                }
            }
            SETTINGS_MENU => {
                if let Err(err) = show_window(&app_handle.get_window(settings::SCREEN)) {
                    log::error!("Unable to show settings window from tray. {err}");
                }
            }
            _ => {}
        },
        _ => {}
    }
}
