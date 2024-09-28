use crate::primary;
use crate::utils::window::show_window;
use ccv_contract::models::{EventPayload, MainShortcutPressedPayload, Settings};
use std::sync::mpsc::Receiver;
use tauri::{async_runtime, AppHandle, Window};

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux::main_loop_hotkey_change;

#[cfg(not(target_os = "linux"))]
mod non_linux;

#[cfg(not(target_os = "linux"))]
pub use non_linux::main_loop_hotkey_change;

pub fn listen_hotkey_change(
    app_handle: AppHandle,
    init_settings: Settings,
    settings_receiver: Receiver<Settings>,
) -> () {
    async_runtime::spawn(async move {
        if let Err(err) = main_loop_hotkey_change(app_handle, init_settings, settings_receiver) {
            log::error!("Error on listening hotkeys. {err}");
        }
    });
}

fn activate_primary_window(primary_window_option: &Option<Window>) -> () {
    if let Some(primary_window) = primary_window_option {
        let was_visible_before = primary_window.is_visible().unwrap_or(false);
        if let Err(err) = show_window(primary_window_option) {
            log::error!("Unable to show primary window by shortcut. {err}");
        } else {
            if let Err(err) = primary_window.emit(
                primary::MAIN_SHORTCUT_PRESSED_EVENT,
                EventPayload {
                    data: MainShortcutPressedPayload {
                        changed_from_hidden_to_visile: !was_visible_before,
                    },
                },
            ) {
                log::error!("Unable to send shortcut pressed event. {err}");
            }
        }
    }
}
