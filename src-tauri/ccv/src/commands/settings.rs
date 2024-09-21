use crate::events::{ITEMS_CHANGED, SETTINGS_UPDATED};
use crate::screens::{MAIN, SETTINGS};
use crate::state::CopyItemState;
use crate::{commands::main::insert_copy_item_if_not_found, state::SettingsState};
use ccv_contract::models::Shortcut;
use ccv_contract::{
    app_error,
    error::{log_error, AppError},
    models::{EventPayload, Settings},
};
use chrono::{DateTime, Utc};
use tauri::{command, AppHandle, GlobalShortcutManager, Manager, State};
use tauri_plugin_clipboard::ClipboardManager;

use super::utils::show_window;

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
    let should_update_shortcuts = settings.is_some()
        && settings.clone().unwrap().keybindings.open_ccv != new_settings.keybindings.open_ccv;

    *settings = Some(new_settings.clone());

    let app_data_dir = app.app_handle().path_resolver().app_data_dir().unwrap();
    log_error(
        SettingsState::write_settings(&app_data_dir, &new_settings),
        "Unable to save settings",
    )?;

    if should_update_shortcuts {
        log_error(
            app.global_shortcut_manager().unregister_all(),
            "Unable to unregister shortcut",
        )?;
        log_error(
            register_keybindings(&app, &new_settings),
            "Unable to register shortcut",
        )?;
    }

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
        log_error(show_window(&window), "Unable to show settings")
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

pub fn register_keybindings(app: &AppHandle, settings: &Settings) -> Result<(), AppError> {
    let keys = parse_shortcut(&settings.keybindings.open_ccv)?;
    let accelerator = keys.join(" + ");

    if let Some(main_window) = app.get_window(MAIN) {
        app.global_shortcut_manager()
            .register(accelerator.as_str(), move || {
                println!("show !");
                log_error(show_window(&main_window), "Unable to show main window").unwrap();
            })
            .map_err(|err| app_error!("{err}"))
    } else {
        Err(app_error!("Main window option returns None"))
    }
}

fn parse_shortcut(shortcut: &Shortcut) -> Result<Vec<String>, AppError> {
    let mut result = vec![];

    if shortcut.ctrl_key {
        result.push("Ctrl".to_string());
    }
    if shortcut.alt_key {
        result.push("Alt".to_string());
    }
    if shortcut.shift_key {
        result.push("Shift".to_string());
    }
    if shortcut.meta_key {
        result.push("Super".to_string());
    }
    if let Some(code) = shortcut.code.clone() {
        if let Some(value) = get_key_maps(code.as_str()) {
            result.push(value.to_string());
        }
    }

    Ok(result)
}

fn get_key_maps(code: &str) -> Option<&'static str> {
    match code {
        "Minus" => Some("-"),
        "Equal" => Some("="),
        "Slash" => Some("Slash"),
        "BracketLeft" => Some("BracketLeft"),
        "BracketRight" => Some("BracketRight"),
        "SemiColon" => Some(";"),
        "Quote" => Some("'"),
        "Backslash" => Some("Backslash"),
        "Comma" => Some(","),
        "Dot" => Some("."),
        "Backquote" => Some("Backquote"),
        "Backspace" => Some("Backspace"),
        "CapsLock" => Some("CapsLock"),
        "Insert" => Some("Insert"),
        "Delete" => Some("Delete"),
        "DownArrow" => Some("DownArrow"),
        "UpArrow" => Some("UpArrow"),
        "LeftArrow" => Some("LeftArrow"),
        "RightArrow" => Some("RightArrow"),
        "End" => Some("End"),
        "Home" => Some("Home"),
        "Escape" => Some("Escape"),
        "Enter" => Some("Enter"),
        "PageDown" => Some("PageDown"),
        "PageUp" => Some("PageUp"),
        "Space" => Some("Space"),
        "Tab" => Some("Tab"),
        "ScrollLock" => Some("ScrollLock"),
        "Pause" => Some("Pause"),
        "ContextMenu" => Some("ContextMenu"),
        "NumLock" => Some("NumLock"),
        "F1" => Some("F1"),
        "F2" => Some("F2"),
        "F3" => Some("F3"),
        "F4" => Some("F4"),
        "F5" => Some("F5"),
        "F6" => Some("F6"),
        "F7" => Some("F7"),
        "F8" => Some("F8"),
        "F9" => Some("F9"),
        "F10" => Some("F10"),
        "F11" => Some("F11"),
        "F12" => Some("F12"),
        "Digit0" => Some("0"),
        "Digit1" => Some("1"),
        "Digit2" => Some("2"),
        "Digit3" => Some("3"),
        "Digit4" => Some("4"),
        "Digit5" => Some("5"),
        "Digit6" => Some("6"),
        "Digit7" => Some("7"),
        "Digit8" => Some("8"),
        "Digit9" => Some("9"),
        "NumpadDivide" => Some("NumpadDivide"),
        "NumpadMultiply" => Some("NUMPADCLEAR"),
        "NumpadSubtract" => Some("NumpadSubtract"),
        "NumpadAdd" => Some("NumpadAdd"),
        "NumpadEnter" => Some("NumpadEnter"),
        "NumpadDecimal" => Some("NUMCOMMA"),
        "Numpad0" => Some("Numpad0"),
        "Numpad1" => Some("Numpad1"),
        "Numpad2" => Some("Numpad2"),
        "Numpad3" => Some("Numpad3"),
        "Numpad4" => Some("Numpad4"),
        "Numpad5" => Some("Numpad5"),
        "Numpad6" => Some("Numpad6"),
        "Numpad7" => Some("Numpad7"),
        "Numpad8" => Some("Numpad8"),
        "Numpad9" => Some("Numpad9"),
        "KeyQ" => Some("Q"),
        "KeyW" => Some("W"),
        "KeyE" => Some("E"),
        "KeyR" => Some("R"),
        "KeyT" => Some("T"),
        "KeyY" => Some("Y"),
        "KeyU" => Some("U"),
        "KeyI" => Some("I"),
        "KeyO" => Some("O"),
        "KeyP" => Some("P"),
        "KeyA" => Some("A"),
        "KeyS" => Some("S"),
        "KeyD" => Some("D"),
        "KeyF" => Some("F"),
        "KeyG" => Some("G"),
        "KeyH" => Some("H"),
        "KeyJ" => Some("J"),
        "KeyK" => Some("K"),
        "KeyL" => Some("L"),
        "KeyZ" => Some("Z"),
        "KeyX" => Some("X"),
        "KeyC" => Some("C"),
        "KeyV" => Some("V"),
        "KeyB" => Some("B"),
        "KeyN" => Some("N"),
        "KeyM" => Some("M"),
        _ => None,
    }
}
