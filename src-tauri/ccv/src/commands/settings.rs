use crate::events::{ITEMS_CHANGED, SETTINGS_UPDATED};
use crate::screens::SETTINGS;
use crate::state::CopyItemState;
use crate::{commands::main::insert_copy_item_if_not_found, state::SettingsState};
use ccv_contract::{
    app_error,
    error::{log_error, AppError},
    models::{EventPayload, Settings},
};
use chrono::{DateTime, Utc};
use tauri::{command, AppHandle, Manager, State};
use tauri_plugin_clipboard::ClipboardManager;

#[command]
pub fn get_settings(state: State<SettingsState>) -> Result<Option<Settings>, AppError> {
    let settings = state.settings.lock().unwrap();
    Ok(settings.clone())
}

#[command]
pub fn set_settings(
    new_settings: Settings,
    app: AppHandle,
    state: State<SettingsState>,
) -> Result<(), AppError> {
    let mut settings = state.settings.lock().unwrap();
    *settings = Some(new_settings.clone());

    let app_data_dir = app.app_handle().path_resolver().app_data_dir().unwrap();

    log_error(
        SettingsState::write_settings(&app_data_dir, &new_settings),
        "Unable to save settings",
    )?;

    log_error(
        app.emit_all(
            SETTINGS_UPDATED,
            EventPayload {
                data: settings.clone(),
            },
        ),
        "Unable to send event",
    )
}

#[command]
pub fn hide_settings_window(app: AppHandle) -> Result<(), AppError> {
    let window = app.get_window(SETTINGS);
    if let Some(window) = window {
        log_error(window.hide(), "Unable to hide settings window")?;
        Ok(())
    } else {
        log_error(
            Err(app_error!("Window option returns None")),
            "Unable to find settings window",
        )
    }
}

#[command]
pub fn show_settings_window(app: AppHandle) -> Result<(), AppError> {
    let window = app.get_window(SETTINGS);
    if let Some(window) = window {
        log_error(window.show(), "Unable to show settings window")?;
        log_error(window.set_focus(), "Unable to focus settings window")?;
        Ok(())
    } else {
        log_error(
            Err(app_error!("Window option returns None")),
            "Unable to find settings window",
        )
    }
}

#[command]
pub fn remove_copy_items(
    item_ids: String,
    app: AppHandle,
    state: State<CopyItemState>,
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
        insert_copy_item_if_not_found(repository.as_ref(), state_clipboard),
        "Unable to insert item after deletion",
    )?;

    log_error(
        app.emit_all(
            ITEMS_CHANGED,
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
    app: AppHandle,
    state: State<CopyItemState>,
    state_clipboard: State<ClipboardManager>,
) -> Result<(), AppError> {
    let repository = state.repository.lock().unwrap();
    log_error(
        repository.remove_older(sinse),
        "Error on removing old copy items",
    )?;
    log_error(
        insert_copy_item_if_not_found(repository.as_ref(), state_clipboard),
        "Unable to insert item after deletion",
    )?;

    log_error(
        app.emit_all(
            ITEMS_CHANGED,
            EventPayload {
                data: format!("Delete items older {sinse}"),
            },
        ),
        "Unable to send event",
    )?;

    Ok(())
}
