use crate::primary;
use crate::settings;

use crate::primary::state::PrimaryState;
use crate::settings::state::SettingsState;
use crate::utils::window::{hide_window, show_window};
use ccv_contract::{
    app_error,
    error::{log_error, AppError},
    models::{EventPayload, Settings},
};
use chrono::{DateTime, Utc};
use tauri::{command, AppHandle, GlobalShortcutManager, Manager, State};
use tauri_plugin_clipboard::ClipboardManager;

#[command]
pub fn get_settings(state: State<SettingsState>) -> Result<Option<Settings>, AppError> {
    let settings = state.settings.lock().unwrap();
    Ok(settings.clone())
}

#[command]
pub fn set_settings(
    new_settings: Settings,
    app_handle: AppHandle,
    state: State<SettingsState>,
) -> Result<(), AppError> {
    let mut settings = state.settings.lock().unwrap();
    let should_update_shortcuts = settings.is_some()
        && settings.clone().unwrap().keybindings.open_ccv != new_settings.keybindings.open_ccv;

    *settings = Some(new_settings.clone());

    let app_data_dir = app_handle
        .app_handle()
        .path_resolver()
        .app_data_dir()
        .unwrap();
    log_error(
        settings::core::write_settings(&app_data_dir, &new_settings),
        "Unable to save settings",
    )?;

    if should_update_shortcuts {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            log_error(
                app_handle.global_shortcut_manager().unregister_all(),
                "Unable to unregister shortcut",
            )?;
            log_error(
                settings::core::register_keybindings(&app_handle, &new_settings),
                "Unable to register shortcut",
            )?;
        }

        // TODO change to linux
        #[cfg(target_os = "linux")]
        {
            log_error(
                state
                    .hotkey_change
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .send(new_settings.clone()),
                "Unable to register shortcut",
            )?;
        }
    }

    log_error(
        app_handle.emit_all(
            settings::SETTINGS_UPDATED_EVENT,
            EventPayload {
                data: settings.clone(),
            },
        ),
        "Unable to send event",
    )
}

#[command]
pub fn hide_settings_window(app_handle: AppHandle) -> Result<(), AppError> {
    log_error(
        hide_window(&app_handle.get_window(settings::SCREEN)),
        "Unable to hide settings window",
    )
}

#[command]
pub fn show_settings_window(app_handle: AppHandle) -> Result<(), AppError> {
    log_error(
        show_window(&app_handle.get_window(settings::SCREEN)),
        "Unable to show settings window",
    )
}

#[command]
pub fn remove_copy_items(
    item_ids: String,
    app_handle: AppHandle,
    state: State<PrimaryState>,
    state_clipboard: State<ClipboardManager>,
) -> Result<(), AppError> {
    let splitted_item_ids: Vec<&str> = item_ids.split(",").into_iter().map(|x| x.trim()).collect();
    if splitted_item_ids.is_empty() {
        return Err(app_error!("No id specified"));
    }
    let repository = state.repository.lock().unwrap();
    log_error(
        repository.remove(&splitted_item_ids),
        "Error on removing copy items",
    )?;
    log_error(
        primary::core::insert_copy_item_if_not_found(repository.as_ref(), state_clipboard),
        "Unable to insert item after deletion",
    )?;

    log_error(
        app_handle.emit_all(
            primary::ITEMS_CHANGED_EVENT,
            EventPayload {
                data: format!("Delete items with ids '{item_ids}'"),
            },
        ),
        "Unable to send event",
    )?;

    Ok(())
}

#[command]
pub fn remove_copy_items_older(
    sinse: DateTime<Utc>,
    app_handle: AppHandle,
    state: State<PrimaryState>,
    state_clipboard: State<ClipboardManager>,
) -> Result<(), AppError> {
    let repository = state.repository.lock().unwrap();
    log_error(
        repository.remove_older(sinse),
        "Error on removing old copy items",
    )?;
    log_error(
        primary::core::insert_copy_item_if_not_found(repository.as_ref(), state_clipboard),
        "Unable to insert item after deletion",
    )?;

    log_error(
        app_handle.emit_all(
            primary::ITEMS_CHANGED_EVENT,
            EventPayload {
                data: format!("Delete items older {sinse}"),
            },
        ),
        "Unable to send event",
    )?;

    Ok(())
}
