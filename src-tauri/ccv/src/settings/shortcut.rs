use crate::{primary, settings, utils::window::show_window};
use ccv_contract::{
    app_error,
    error::AppError,
    models::{AllShortcuts, EventPayload, MainShortcutPressedPayload, Shortcut},
};
use tauri::{AppHandle, WebviewWindow};
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{
    Code, GlobalShortcutExt, Modifiers, Shortcut as HotKey, ShortcutState,
};

pub fn register_shortcuts(
    app_handle: &AppHandle,
    all_shortcuts: &AllShortcuts,
    unregister_all: bool,
) -> Result<(), AppError> {
    if unregister_all {
        app_handle
            .global_shortcut()
            .unregister_all()
            .map_err(|err| app_error!("Unable to unregister shortcuts. {err}"))?
    }

    if let Some(open_ccv_shortcut) = get_shortcut(&all_shortcuts.open_ccv) {
        app_handle
            .global_shortcut()
            .register(open_ccv_shortcut)
            .map_err(|err| app_error!("Unable to register open ccv shortcut. {err}"))?;
    }

    Ok(())
}

pub fn register_handlers(app_handle: &AppHandle) -> Result<(), AppError> {
    app_handle
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app_handle, shortcut, event| match event.state() {
                    ShortcutState::Pressed => {
                        let settings_state = app_handle.state::<settings::state::SettingsState>();
                        let settings = settings_state.settings.lock().unwrap();

                        if let Some(settings) = settings.as_ref() {
                            if let Some(open_ccv_shortcut) =
                                settings::shortcut::get_shortcut(&settings.all_shortcuts.open_ccv)
                            {
                                if shortcut == &open_ccv_shortcut {
                                    activate_primary_window(
                                        &app_handle.get_webview_window(primary::SCREEN),
                                    )
                                }
                            }
                        }
                    }
                    ShortcutState::Released => {}
                })
                .build(),
        )
        .map_err(|err| app_error!("Unable to initialize global shortcut plugin. {err}"))
}

fn activate_primary_window(primary_window_option: &Option<WebviewWindow>) -> () {
    if let Some(primary_window) = primary_window_option {
        let was_visible_before = primary_window.is_visible().unwrap_or(false);
        if let Err(err) = show_window(primary_window_option) {
            log::error!("Unable to show primary window by shortcut. {err}");
        } else {
            if let Err(err) = primary_window.emit_to(
                primary::SCREEN,
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

    // for some reason on linux app doesnt show form the first attempt
    #[cfg(target_os = "linux")]
    {
        let copy = primary_window_option.clone();

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            if let Some(primary_window) = copy {
                if let Err(err) = show_window(&Some(primary_window.clone())) {
                    log::error!("Unable to show primary window by shortcut. {err}");
                }
            }
        });
    }
}

pub fn get_shortcut(shortcut: &Shortcut) -> Option<HotKey> {
    if let Some(code) = shortcut.code.as_ref() {
        if let Some(code) = get_shortcut_code(code.as_str()) {
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
                modifier = modifier | Modifiers::SUPER;
            }

            if modifier == Modifiers::empty() {
                Some(HotKey::new(None, code))
            } else {
                Some(HotKey::new(Some(modifier), code))
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn get_shortcut_code(code: &str) -> Option<Code> {
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
