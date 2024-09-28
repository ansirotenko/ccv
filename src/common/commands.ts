import { invoke } from '@tauri-apps/api/tauri';
import { AboutData, CopyCategory, CopyItem, SearchResult, Settings } from './contract';

export async function getAboutData() {
    return await invoke<AboutData>('get_about_data');
}

export async function openAnything(target: string) {
    return await invoke<void>('open_uri', { target: target });
}

export async function hideAboutWindow() {
    await invoke<void>('hide_about_window');
}

export async function showAboutWindow() {
    await invoke<void>('show_about_window');
}

export async function hideSettingsWindow() {
    await invoke<void>('hide_settings_window');
}

export async function setSettings(settings: Settings) {
    await invoke<void>('set_settings', { newSettings: settings });
}

export async function showSettingsWindow() {
    await invoke<void>('show_settings_window');
}

export async function removeCopyItems(itemIds: string) {
    await invoke<void>('remove_copy_items', { itemIds: itemIds });
}

export async function removeCopyItemsOlder(sinse: Date) {
    await invoke<void>('remove_copy_items_older', { sinse: sinse });
}

type CopyItemRaw = Omit<CopyItem, 'lastReuse' | 'created'> & {
    lastReuse: string;
    created: string;
};
type SearchResultRaw = Omit<SearchResult, 'items'> & {
    items: CopyItemRaw[];
};

export async function searchCopyItems(query: string | null, page: number, pageSize: number, categories: CopyCategory[]) {
    const searchResultRaw = await invoke<SearchResultRaw>('search_copy_items', {
        query: query,
        page: page,
        pageSize: pageSize,
        categories: categories,
    });
    const searchResult: SearchResult = {
        ...searchResultRaw,
        items: searchResultRaw.items.map((r) => toCopyItem(r)),
    };
    return searchResult;
}

export async function insertCopyItem() {
    const rawCopyItem = await invoke<CopyItemRaw>('insert_copy_item');
    return toCopyItem(rawCopyItem);
}

function toCopyItem(rawCopyItem: CopyItemRaw): CopyItem {
    return {
        ...rawCopyItem,
        lastReuse: new Date(rawCopyItem.lastReuse),
        created: new Date(rawCopyItem.created),
    };
}

export async function reuseCopyItem(itemId: string) {
    const rawCopyItem = await invoke<CopyItemRaw>('reuse_copy_item', { itemId: itemId });
    return toCopyItem(rawCopyItem);
}

export async function hidePrimaryWindow() {
    await invoke<void>('hide_primary_window');
}

export async function showPrimaryWindow() {
    await invoke<void>('show_primary_window');
}
