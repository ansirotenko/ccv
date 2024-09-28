import { invoke } from '@tauri-apps/api/tauri';
import { AboutData } from '../api';

async function getAboutData() {
    return await invoke<AboutData>('get_about_data');
}

async function open(target: string) {
    return await invoke<void>('open', { target: target });
}

async function hideAboutWindow() {
    await invoke<void>('hide_about_window');
}

export function useBackend() {
    return {
        getAboutData,
        open,
        hideAboutWindow,
    };
}
