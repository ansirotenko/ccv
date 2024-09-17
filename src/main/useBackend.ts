import { useEffect, useRef } from 'react';
import { onSomethingUpdate, startListening } from 'tauri-plugin-clipboard-api';
import { invoke } from '@tauri-apps/api/tauri';
import { CopyCategory, CopyItem, EventPayload, SearchResult } from '../api';
import { emit, listen as subscribe } from '@tauri-apps/api/event';
import { ITEMS_CHANGED, HIGHLIGHT_REPORT_BUG } from '../events';

type CopyItemRaw = Omit<CopyItem, 'lastReuse' | 'created'> & {
    lastReuse: string;
    created: string;
};
type SearchResultRaw = Omit<SearchResult, 'items'> & {
    items: CopyItemRaw[];
};
type ActivatedFunc = (patch: CopyItem) => void;
type ItemsChangedHandler = () => void;
type UnlistendClipboardFunc = () => Promise<void>;

async function searchCopyItems(query: string | null, page: number, pageSize: number, categories: CopyCategory[]) {
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

async function insertCopyItem() {
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

async function hideMainWindow() {
    await invoke<void>('hide_main_window');
}

async function showMainWindow() {
    await invoke<void>('show_main_window');
}

async function showSettingsWindow() {
    await invoke<void>('show_settings_window');
}

async function showAboutWindow() {
    await invoke<void>('show_about_window');
    await emit(HIGHLIGHT_REPORT_BUG, { data: 'Highlight bug report' } as EventPayload<string>);
}

export function useBackend(onClipboardChange: ActivatedFunc, onItemsChanged: ItemsChangedHandler) {
    const onChangeRef = useRef<ActivatedFunc>();
    onChangeRef.current = onClipboardChange;

    const itemsChangedHandler = useRef<ItemsChangedHandler>();
    itemsChangedHandler.current = onItemsChanged;

    const unlistenSomethingUpdatedRef = useRef<UnlistendClipboardFunc>();

    async function reuseCopyItem(itemId: string) {
        await stopListenClipboard();
        const rawCopyItem = await invoke<CopyItemRaw>('reuse_copy_item', { itemId: itemId });
        await startListenClipboard();
        return toCopyItem(rawCopyItem);
    }

    async function startListenClipboard() {
        const unlistenClipboard = await startListening();
        const unlistenSomethingUpdated = await onSomethingUpdate(async () => {
            if (onChangeRef.current) {
                const newItem = await insertCopyItem();
                onChangeRef.current(newItem);
            }
        });

        unlistenSomethingUpdatedRef.current = async () => {
            unlistenSomethingUpdated();
            await unlistenClipboard();
        };
    }

    async function stopListenClipboard() {
        if (unlistenSomethingUpdatedRef.current) {
            await unlistenSomethingUpdatedRef.current();
            unlistenSomethingUpdatedRef.current = undefined;
        }
    }

    useEffect(() => {
        async function listen() {
            const unsubscribeItemsChanged = await subscribe<EventPayload<string>>(ITEMS_CHANGED, () => {
                if (itemsChangedHandler.current) {
                    itemsChangedHandler.current();
                }
            });

            await startListenClipboard();

            return async () => {
                await stopListenClipboard();
                unsubscribeItemsChanged();
            };
        }

        const promise = listen();
        return () => {
            promise.then((c) => c());
        };
    }, []);

    return {
        showMainWindow,
        hideMainWindow,
        showSettingsWindow,
        insertCopyItem,
        reuseCopyItem,
        searchCopyItems,
        showAboutWindow,
    };
}
