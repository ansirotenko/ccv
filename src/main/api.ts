import { invoke } from '@tauri-apps/api/tauri';

export interface CopyItem {
    id: string;
    displayName: string;
}

export async function searchCopyItems(query: string | null) {
    return await invoke<CopyItem[]>('search_copy_items', { query: query || '' });
}
