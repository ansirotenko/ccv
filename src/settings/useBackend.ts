import { invoke } from '@tauri-apps/api/tauri';
import { Settings, Shortcut } from '../api';

async function hideSettingsWindow() {
    await invoke<void>('hide_settings_window');
}

async function removeCopyItems(itemIds: string) {
    await invoke<void>('remove_copy_items', { itemIds: itemIds });
}

async function removeCopyItemsOlder(sinse: Date) {
    await invoke<void>('remove_copy_items_older', { sinse: sinse });
}

async function setSettings(settings: Settings) {
    await invoke<void>('set_settings', { newSettings: settings });
}

function getShortcutDisplay(shortcut: Shortcut) {
    let keys: string[] = [];
    if (shortcut.ctrlKey) {
        keys.push('Ctrl');
    }
    if (shortcut.altKey) {
        keys.push('Alt');
    }
    if (shortcut.shiftKey) {
        keys.push('Shift');
    }
    if (shortcut.metaKey) {
        keys.push('Meta');
    }
    if (shortcut.code && shortcut.code in keyMapByCode) {
        keys.push(keyMapByCode[shortcut.code]);
    }

    return keys.join(" + ");
}

export function useBackend() {
    return {
        hideSettingsWindow,
        removeCopyItems,
        removeCopyItemsOlder,
        setSettings,
        getShortcutDisplay
    };
}

const keyMapByCode: Record<string, string> = {
    "Minus": "-", 
    "Equal": "=", 
    "Slash": "/", 
    "BracketLeft": "[", 
    "BracketRight": "]", 
    "SemiColon": ";", 
    "Quote": "'", 
    "Backslash": "\\", 
    "Comma": ",", 
    "Dot": ".", 
    "Backquote": "`",
    "Backspace": "Backspace",
    "CapsLock": "CapsLock",
    "Insert": "Insert",
    "Delete": "Delete",
    "DownArrow": "DownArrow",
    "UpArrow": "UpArrow",
    "LeftArrow" : "LeftArrow",
    "RightArrow": "RightArrow",
    "End": "End",
    "Home": "Home",
    "Escape": "Escape",
    "Enter": "Enter",
    "PageDown": "PageDown",
    "PageUp": "PageUp",
    "Space": "Space",
    "Tab": "Tab",
    "ScrollLock": "ScrollLock",
    "Pause": "Pause",
    "NumLock": "NumLock",
    "F1": "F1",
    "F2": "F2",
    "F3": "F3",
    "F4": "F4",
    "F5": "F5",
    "F6": "F6",
    "F7": "F7",
    "F8": "F8",
    "F9": "F9",
    "F10": "F10",
    "F11": "F11",
    "F12": "F12",
    "Digit0": "0",
    "Digit1": "1",
    "Digit2": "2",
    "Digit3": "3",
    "Digit4": "4",
    "Digit5": "5",
    "Digit6": "6",
    "Digit7": "7",
    "Digit8": "8",
    "Digit9": "9",
    "KeyQ": "Q",
    "KeyW": "W",
    "KeyE": "E",
    "KeyR": "R",
    "KeyT": "T",
    "KeyY": "Y",
    "KeyU": "U",
    "KeyI": "I",
    "KeyO": "O",
    "KeyP": "P",
    "KeyA": "A",
    "KeyS": "S",
    "KeyD": "D",
    "KeyF": "F",
    "KeyG": "G",
    "KeyH": "H",
    "KeyJ": "J",
    "KeyK": "K",
    "KeyL": "L",
    "KeyZ": "Z",
    "KeyX": "X",
    "KeyC": "C",
    "KeyV": "V",
    "KeyB": "B",
    "KeyN": "N",
    "KeyM": "M",
    // TODO make this work
    "NumpadDivide": "/", 
    "NumpadMultiply": "*", 
    "NumpadSubtract": "-", 
    "NumpadAdd": "+", 
    "NumpadEnter": "Enter",
    "NumpadDecimal": ".",
    "Numpad0": "0",
    "Numpad1": "1",
    "Numpad2": "2",
    "Numpad3": "3",
    "Numpad4": "4",
    "Numpad5": "5",
    "Numpad6": "6",
    "Numpad7": "7",
    "Numpad8": "8",
    "Numpad9": "9",
}