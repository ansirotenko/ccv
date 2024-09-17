use std::collections::{HashMap, HashSet};

use ccv_contract::{app_error, error::AppError};
use rdev::{listen, EventType, Key};
use tauri::{Manager, Window};

use crate::{commands::utils::show_window, state::SettingsState};

pub fn listen_shortcut(main_window: Window) -> Result<(), AppError> {
    let key_maps: HashMap<&str, Key> = HashMap::from([
        ("Ctrl", Key::ControlLeft),
        ("Shift", Key::ShiftLeft),
        ("0", Key::Num0),
        ("1", Key::Num1),
        ("2", Key::Num2),
        ("3", Key::Num3),
        ("4", Key::Num4),
        ("5", Key::Num5),
        ("6", Key::Num6),
        ("7", Key::Num7),
        ("8", Key::Num8),
        ("9", Key::Num9),
        ("F1", Key::F1),
        ("F2", Key::F2),
        ("F3", Key::F3),
        ("F4", Key::F4),
        ("F5", Key::F5),
        ("F6", Key::F6),
        ("F7", Key::F7),
        ("F8", Key::F8),
        ("F9", Key::F9),
        ("F10", Key::F10),
        ("F11", Key::F11),
        ("F12", Key::F12),
        ("Alt", Key::Alt),
        ("BACKSPACE", Key::Backspace),
        ("CAPSLOCk", Key::CapsLock),
        ("INSERT", Key::Insert),
        ("DELETE", Key::Delete),
        ("ARROWDOWN", Key::DownArrow),
        ("ARROWUP", Key::UpArrow),
        ("ARROWLEFT", Key::LeftArrow),
        ("ARROWRIGHT", Key::RightArrow),
        ("END", Key::End),
        ("HOME", Key::Home),
        ("ESCAPE", Key::Escape),
        ("PAGEDOWN", Key::PageDown),
        ("PAGEUP", Key::PageUp),
        ("ENTER", Key::Return),
        ("Space", Key::Space),
        ("Space", Key::Space),
        ("TAB", Key::Tab),
        ("SCROLLLOCK", Key::ScrollLock),
        ("PAUSE", Key::Pause),
        ("NUMLOCK", Key::NumLock),
        ("Backquote", Key::BackQuote),
        ("-", Key::Minus),
        ("=", Key::Equal),
        ("/", Key::Slash),
        ("Q", Key::KeyQ),
        ("W", Key::KeyW),
        ("E", Key::KeyE),
        ("R", Key::KeyR),
        ("T", Key::KeyT),
        ("Y", Key::KeyY),
        ("U", Key::KeyU),
        ("I", Key::KeyI),
        ("O", Key::KeyO),
        ("P", Key::KeyP),
        ("[", Key::LeftBracket),
        ("]", Key::RightBracket),
        ("A", Key::KeyA),
        ("S", Key::KeyS),
        ("D", Key::KeyD),
        ("F", Key::KeyF),
        ("G", Key::KeyG),
        ("H", Key::KeyH),
        ("J", Key::KeyJ),
        ("K", Key::KeyK),
        ("L", Key::KeyL),
        (";", Key::SemiColon),
        ("'", Key::Quote),
        ("\\", Key::BackSlash),
        ("Z", Key::KeyZ),
        ("X", Key::KeyX),
        ("C", Key::KeyC),
        ("V", Key::KeyV),
        ("B", Key::KeyB),
        ("N", Key::KeyN),
        ("M", Key::KeyM),
        (",", Key::Comma),
        (".", Key::Dot),
    ]);

    let alt_key_maps: HashMap<&str, Key> = HashMap::from([
        ("Ctrl", Key::ControlRight),
        ("Shift", Key::ShiftRight),
        ("ENTER", Key::KpReturn),
        ("-", Key::KpMinus),
        ("+", Key::KpPlus),
        ("+", Key::KpPlus),
        ("*", Key::KpMultiply),
        ("/", Key::KpDivide),
        ("0", Key::Kp0),
        ("1", Key::Kp1),
        ("2", Key::Kp2),
        ("3", Key::Kp3),
        ("4", Key::Kp4),
        ("5", Key::Kp5),
        ("6", Key::Kp6),
        ("7", Key::Kp7),
        ("8", Key::Kp8),
        ("9", Key::Kp9),
        ("DELETE", Key::Delete),
    ]);

    let mut pressed_keys: HashSet<Key> = HashSet::new();
    // This will block.
    listen(move |event| match event.event_type {
        EventType::KeyPress(key) => {
            pressed_keys.insert(key);
            let settings_state = main_window.state::<SettingsState>();
            let settings = settings_state.settings.lock().unwrap().clone();
            if let Some(settings) = settings {
                let is_match = settings.keybindings.open_ccv.iter().all(|key| {
                    let key = key.as_str();
                    if let Some(mk) = key_maps.get(key) {
                        if pressed_keys.contains(mk) {
                            return true;
                        }
                    }
                    if let Some(amk) = alt_key_maps.get(key) {
                        if pressed_keys.contains(amk) {
                            return true;
                        }
                    }
                    return false;
                });
                if is_match {
                    if let Err(err) = show_window(&main_window) {
                        log::error!("Unable to show main window. {err}");
                    }
                }
            }
        }
        EventType::KeyRelease(key) => {
            pressed_keys.remove(&key);
        }
        _ => {}
    })
    .map_err(|_| app_error!("Failed to listen"))
}
