use crate::about;
use crate::primary;
use crate::settings;
use crate::tray::menus::{ABOUT_MENU, QUIT_MENU, SETTINGS_MENU, SHOW_MENU};
use crate::utils::window::show_window;
use ccv_contract::app_error;
use ccv_contract::error::AppError;
use tauri::AppHandle;
use tauri::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    Error, Manager, Runtime,
};

pub fn build(app_handle: &AppHandle) -> Result<(), AppError> {
    let menu = get_menu(app_handle).map_err(|err| app_error!("Failed to build menu. {err}"))?;

    let builder = TrayIconBuilder::new()
        .icon(app_handle.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(menu_event_handler)
        .on_tray_icon_event(tray_icon_event_handler);

    builder
        .build(app_handle.app_handle())
        .map_err(|err| app_error!("Tray builder error. {err}"))?;

    #[cfg(target_os = "macos")]
    {
        app_handle
            .set_activation_policy(tauri::ActivationPolicy::Accessory)
            .map_err(|err| app_error!("Tray builder error. {err}"))?;
    }

    Ok(())
}

fn get_menu<R: Runtime>(app_handle: &AppHandle<R>) -> Result<Menu<R>, Error> {
    let about = MenuItem::with_id(
        app_handle,
        ABOUT_MENU.to_string(),
        "About",
        true,
        None::<&str>,
    )?;
    let settings = MenuItem::with_id(
        app_handle,
        SETTINGS_MENU.to_string(),
        "Settings",
        true,
        None::<&str>,
    )?;
    let show = MenuItem::with_id(
        app_handle,
        SHOW_MENU.to_string(),
        "Show",
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(
        app_handle,
        QUIT_MENU.to_string(),
        "Quit",
        true,
        None::<&str>,
    )?;
    let separator = PredefinedMenuItem::separator(app_handle)?;

    Menu::with_items(
        app_handle,
        &[
            &show, &separator, &settings, &separator, &about, &separator, &quit,
        ],
    )
}

fn menu_event_handler(app_handle: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        QUIT_MENU => {
            app_handle.exit(0);
        }
        ABOUT_MENU => {
            if let Err(err) = show_window(&app_handle.get_webview_window(about::SCREEN)) {
                log::error!("Unable to show about window from tray. {err}");
            }
        }
        SHOW_MENU => {
            if let Err(err) = show_window(&app_handle.get_webview_window(primary::SCREEN)) {
                log::error!("Unable to show primary window from tray. {err}");
            }
        }
        SETTINGS_MENU => {
            if let Err(err) = show_window(&app_handle.get_webview_window(settings::SCREEN)) {
                log::error!("Unable to show settings window from tray. {err}");
            }
        }
        _ => {}
    }
}

fn tray_icon_event_handler(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::DoubleClick { .. } => {
            if let Err(err) = show_window(&tray.app_handle().get_webview_window(primary::SCREEN)) {
                log::error!("Unable to show primary window from tray. {err}");
            }
        }
        _ => {}
    }
}
