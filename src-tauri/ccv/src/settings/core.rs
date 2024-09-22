use ccv_contract::{
    app_error,
    error::AppError,
    models::{Keybindings, Settings, Shortcut, Theme},
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
use tauri::{AppHandle, Manager};

// TODO change to linux
#[cfg(target_os = "linux")]
use global_hotkey::hotkey::{Code, HotKey, Modifiers};

const SETTINGS_FILE_NAME: &str = "settings.json";

pub fn read_settings(app_data_dir: &PathBuf) -> Result<Settings, AppError> {
    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(app_data_dir.join(SETTINGS_FILE_NAME))
        .map_err(|err| app_error!("Unable to open settings file. {err}"))?;

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .map_err(|err| app_error!("Unable to read settings file. {err}"))?;

    if let Ok(result) = serde_json::from_str(&file_content) {
        Ok(result)
    } else {
        let default_settings = Settings {
            theme: Theme::Light,
            keybindings: Keybindings {
                open_ccv: Shortcut {
                    alt_key: true,
                    ctrl_key: false,
                    meta_key: false,
                    shift_key: false,
                    code: Some("KeyV".to_string()),
                },
            },
        };

        write_settings(app_data_dir, &default_settings)?;
        Ok(default_settings)
    }
}

pub fn write_settings(app_data_dir: &PathBuf, settings: &Settings) -> Result<(), AppError> {
    let json = serde_json::to_string_pretty(settings)
        .map_err(|err| app_error!("Unable to serialize settings. {err}"))?;

    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(app_data_dir.join(SETTINGS_FILE_NAME))
        .map_err(|err| app_error!("Unable to open settings file. {err}"))?;

    file.write_all(json.as_bytes())
        .map_err(|err| app_error!("Unable to write settings file content. {err}"))
}

// TODO change to linux
#[cfg(target_os = "linux")]
pub fn get_hotkey(shortcut: &Shortcut) -> Result<HotKey, AppError> {
    let mut modifier: Modifiers = Modifiers::empty();

    if shortcut.ctrl_key {
        modifier = modifier | Modifiers::CONTROL;
    }
    if shortcut.alt_key {
        modifier = modifier | Modifiers::ALT;
    }
    if shortcut.shift_key {
        modifier = modifier | Modifiers::SHIFT;
    }
    if shortcut.meta_key {
        modifier = modifier | Modifiers::META;
    }
    if let Some(code) = shortcut.code.clone() {
        if let Some(code) = get_hotkey_code(code.as_str()) {
            return if modifier == Modifiers::empty() {
                Ok(HotKey::new(None, code))
            } else {
                Ok(HotKey::new(Some(modifier), code))
            };
        }
    }

    Err(app_error!("Invalid hotkey"))
}

// TODO change to linux
#[cfg(target_os = "linux")]
fn get_hotkey_code(code: &str) -> Option<Code> {
    match code {
        "Minus" => Some(Code::Minus),
        "Equal" => Some(Code::Equal),
        "Slash" => Some(Code::Slash),
        "BracketLeft" => Some(Code::BracketLeft),
        "BracketRight" => Some(Code::BracketRight),
        "SemiColon" => Some(Code::Semicolon),
        "Quote" => Some(Code::Quote),
        "Backslash" => Some(Code::Backslash),
        "Comma" => Some(Code::Comma),
        "Dot" => Some(Code::Period),
        "Backquote" => Some(Code::Backquote),
        "Backspace" => Some(Code::Backspace),
        "CapsLock" => Some(Code::CapsLock),
        "Insert" => Some(Code::Insert),
        "Delete" => Some(Code::Delete),
        "DownArrow" => Some(Code::ArrowDown),
        "UpArrow" => Some(Code::ArrowUp),
        "LeftArrow" => Some(Code::ArrowLeft),
        "RightArrow" => Some(Code::ArrowRight),
        "End" => Some(Code::End),
        "Home" => Some(Code::Home),
        "Escape" => Some(Code::Escape),
        "Enter" => Some(Code::Enter),
        "PageDown" => Some(Code::PageDown),
        "PageUp" => Some(Code::PageUp),
        "Space" => Some(Code::Space),
        "Tab" => Some(Code::Tab),
        "ScrollLock" => Some(Code::ScrollLock),
        "Pause" => Some(Code::Pause),
        "ContextMenu" => Some(Code::ContextMenu),
        "NumLock" => Some(Code::NumLock),
        "F1" => Some(Code::F1),
        "F2" => Some(Code::F2),
        "F3" => Some(Code::F3),
        "F4" => Some(Code::F4),
        "F5" => Some(Code::F5),
        "F6" => Some(Code::F6),
        "F7" => Some(Code::F7),
        "F8" => Some(Code::F8),
        "F9" => Some(Code::F9),
        "F10" => Some(Code::F10),
        "F11" => Some(Code::F11),
        "F12" => Some(Code::F12),
        "Digit0" => Some(Code::Digit0),
        "Digit1" => Some(Code::Digit1),
        "Digit2" => Some(Code::Digit2),
        "Digit3" => Some(Code::Digit3),
        "Digit4" => Some(Code::Digit4),
        "Digit5" => Some(Code::Digit5),
        "Digit6" => Some(Code::Digit6),
        "Digit7" => Some(Code::Digit7),
        "Digit8" => Some(Code::Digit8),
        "Digit9" => Some(Code::Digit9),
        "NumpadDivide" => Some(Code::NumpadDivide),
        "NumpadMultiply" => Some(Code::NumpadMultiply),
        "NumpadSubtract" => Some(Code::NumpadSubtract),
        "NumpadAdd" => Some(Code::NumpadAdd),
        "NumpadEnter" => Some(Code::NumpadEnter),
        "NumpadDecimal" => Some(Code::NumpadComma),
        "Numpad0" => Some(Code::Numpad0),
        "Numpad1" => Some(Code::Numpad1),
        "Numpad2" => Some(Code::Numpad2),
        "Numpad3" => Some(Code::Numpad3),
        "Numpad4" => Some(Code::Numpad4),
        "Numpad5" => Some(Code::Numpad5),
        "Numpad6" => Some(Code::Numpad6),
        "Numpad7" => Some(Code::Numpad7),
        "Numpad8" => Some(Code::Numpad8),
        "Numpad9" => Some(Code::Numpad9),
        "KeyQ" => Some(Code::KeyQ),
        "KeyW" => Some(Code::KeyW),
        "KeyE" => Some(Code::KeyE),
        "KeyR" => Some(Code::KeyR),
        "KeyT" => Some(Code::KeyT),
        "KeyY" => Some(Code::KeyY),
        "KeyU" => Some(Code::KeyU),
        "KeyI" => Some(Code::KeyI),
        "KeyO" => Some(Code::KeyO),
        "KeyP" => Some(Code::KeyP),
        "KeyA" => Some(Code::KeyA),
        "KeyS" => Some(Code::KeyS),
        "KeyD" => Some(Code::KeyD),
        "KeyF" => Some(Code::KeyF),
        "KeyG" => Some(Code::KeyG),
        "KeyH" => Some(Code::KeyH),
        "KeyJ" => Some(Code::KeyJ),
        "KeyK" => Some(Code::KeyK),
        "KeyL" => Some(Code::KeyL),
        "KeyZ" => Some(Code::KeyZ),
        "KeyX" => Some(Code::KeyX),
        "KeyC" => Some(Code::KeyC),
        "KeyV" => Some(Code::KeyV),
        "KeyB" => Some(Code::KeyB),
        "KeyN" => Some(Code::KeyN),
        "KeyM" => Some(Code::KeyM),
        _ => None,
    }
}

#[cfg(not(target_os = "linux"))]
pub fn register_keybindings(app_handle: &AppHandle, settings: &Settings) -> Result<(), AppError> {
    use crate::{primary, utils::window::show_window};
    use ccv_contract::error::log_error;
    use tauri::GlobalShortcutManager;

    let keys = parse_shortcut(&settings.keybindings.open_ccv)?;
    let accelerator = keys.join(" + ");
    let primary_window = app_handle.get_window(primary::SCREEN);

    app_handle
        .global_shortcut_manager()
        .register(accelerator.as_str(), move || {
            log_error(
                show_window(&primary_window),
                "Unable to show primary window",
            )
            .unwrap();
        })
        .map_err(|err| app_error!("{err}"))
}

#[cfg(not(target_os = "linux"))]
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

#[cfg(not(target_os = "linux"))]
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
