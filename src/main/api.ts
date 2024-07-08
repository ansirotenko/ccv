import { invoke } from '@tauri-apps/api/tauri';
import { registerAll } from '@tauri-apps/api/globalShortcut';

export interface CopyItem {
    id: string;
    displayName: string;
}

export async function searchCopyItems(query: string | null) {
    return await invoke<CopyItem[]>('search_copy_items', { query: query || '' });
}

export async function hideWindow() {
    return await invoke<CopyItem[]>('hide_window');
}

export async function showWindow() {
    return await invoke<CopyItem[]>('show_window');
}

export async function listenGlobalEvents() {
    // await registerAll(['CommandOrControl+Shift+C', 'Ctrl+Alt+F12'], (shortcut) => {
    //     console.log(`Shortcut ${shortcut} triggered`);
    // });

    await registerAll(['SHIFT+`'], () => {
        showWindow();
    });
}
