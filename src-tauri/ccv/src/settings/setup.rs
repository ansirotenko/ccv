use crate::settings;
use ccv_contract::{error::AppError, models::Settings};
use std::{path::PathBuf, sync::mpsc::channel};
use tauri::{AppHandle, Manager};
// use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub fn read_settings_and_register_shortcuts(
    app_handle: &AppHandle,
    app_data_dir: &PathBuf,
) -> Result<(), AppError> {
    let state_settings = app_handle.state::<settings::state::SettingsState>();
    let mut settings = state_settings.settings.lock().unwrap();
    match settings::core::read_settings(app_data_dir) {
        Ok(new_settings) => {
            *settings = Some(new_settings);
        }
        Err(err) => {
            log::error!("Unable to read settings file. {err}");
        }
    }

    // let (tx, rx) = channel::<Settings>();
    // TODO restore rx
    let (tx, _) = channel::<Settings>();
    let mut hotkey_change = state_settings.settings_change.lock().unwrap();
    *hotkey_change = Some(tx);

    // TODO test this
    // let settings = settings.clone().unwrap();
    // settings::hotkeys::listen_hotkey_change(app_handle.clone(), settings, rx);



    // let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
    // app.handle().plugin(
    //     tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
    //         println!("{:?}", shortcut);
    //         if shortcut == &ctrl_n_shortcut {
    //             match event.state() {
    //                 ShortcutState::Pressed => {
    //                 println!("Ctrl-N Pressed!");
    //                 }
    //                 ShortcutState::Released => {
    //                 println!("Ctrl-N Released!");
    //                 }
    //             }
    //         }
    //     })
    //     .build(),
    // )?;

    // app.global_shortcut().register(ctrl_n_shortcut)?;

    Ok(())
}
