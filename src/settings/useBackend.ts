import { invoke } from '@tauri-apps/api/tauri';
import { Settings } from '../api';

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


export function useBackend() {
    return {
        hideSettingsWindow,
        removeCopyItems,
        removeCopyItemsOlder,
        setSettings
    };
}