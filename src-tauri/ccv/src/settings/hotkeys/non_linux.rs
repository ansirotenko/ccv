use super::activate_primary_window;
use crate::primary;
use ccv_contract::app_error;
use ccv_contract::models::Settings;
use ccv_contract::{error::AppError, models::Shortcut};
use std::{sync::mpsc::Receiver, thread, time::Duration};
use tauri::{AppHandle, GlobalShortcutManager, Manager};

pub fn main_loop_hotkey_change(
    app_handle: AppHandle,
    init_settings: Settings,
    receiver: Receiver<Settings>,
) -> Result<(), AppError> {
    let mut global_shortcut_manager = app_handle.global_shortcut_manager();
    let mut old_settings = init_settings;

    let primary_window = app_handle.get_window(primary::SCREEN);
    let initial_accelerator = parse_shortcut(&old_settings.all_shortcuts.open_ccv);
    global_shortcut_manager
        .register(initial_accelerator.as_str(), move || {
            activate_primary_window(&primary_window);
        })
        .map_err(|err| app_error!("{err}"))?;

    loop {
        if let Ok(new_settings) = receiver.try_recv() {
            if new_settings != old_settings {
                let old_accelerator = parse_shortcut(&old_settings.all_shortcuts.open_ccv);
                global_shortcut_manager
                    .unregister(old_accelerator.as_str())
                    .map_err(|err| app_error!("{err}"))?;

                let primary_window = app_handle.get_window(primary::SCREEN);
                let new_accelerator = parse_shortcut(&new_settings.all_shortcuts.open_ccv);
                global_shortcut_manager
                    .register(new_accelerator.as_str(), move || {
                        activate_primary_window(&primary_window);
                    })
                    .map_err(|err| app_error!("{err}"))?;

                old_settings = new_settings;
            }
        }

        thread::sleep(Duration::from_millis(50)) // TODO maybe async
    }
}

fn parse_shortcut(shortcut: &Shortcut) -> String {
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

    result.join(" + ")
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
