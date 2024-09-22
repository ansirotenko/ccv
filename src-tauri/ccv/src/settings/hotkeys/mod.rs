use ccv_contract::models::Settings;
use std::sync::mpsc::Receiver;
use tauri::{async_runtime, AppHandle};

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
