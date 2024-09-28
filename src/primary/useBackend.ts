import { useEffect, useRef } from 'react';
import { onSomethingUpdate, startListening } from 'tauri-plugin-clipboard-api';
import { invoke } from '@tauri-apps/api/tauri';
import { CopyCategory, CopyItem, EventPayload, SearchResult } from '../api';
import { emit } from '@tauri-apps/api/event';
import { HIGHLIGHT_REPORT_BUG } from '../events';

type CopyItemRaw = Omit<CopyItem, 'lastReuse' | 'created'> & {
    lastReuse: string;
    created: string;
};
type SearchResultRaw = Omit<SearchResult, 'items'> & {
    items: CopyItemRaw[];
};
type ClipboardChangedHandler = (patch: CopyItem) => void;
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

async function hidePrimaryWindow() {
    await invoke<void>('hide_primary_window');
}

async function showPrimaryWindow() {
    await invoke<void>('show_primary_window');
}

async function showSettingsWindow() {
    await invoke<void>('show_settings_window');
}

async function showAboutWindow() {
    await invoke<void>('show_about_window');
    await emit(HIGHLIGHT_REPORT_BUG, { data: 'Highlight bug report' } as EventPayload<string>); // TODO useRaiseEvent
}

export function useBackend(
    onClipboardChanged: ClipboardChangedHandler
) {
    const clipboardChangeHandler = useRef<ClipboardChangedHandler>();
    clipboardChangeHandler.current = onClipboardChanged;

    const unlistenSomethingUpdatedRef = useRef<UnlistendClipboardFunc>();

    async function reuseCopyItem(itemId: string) {
        await stopListenClipboard();
        const rawCopyItem = await invoke<CopyItemRaw>('reuse_copy_item', { itemId: itemId });
        await startListenClipboard();
        const newItem = toCopyItem(rawCopyItem);

        if (clipboardChangeHandler.current) {
            clipboardChangeHandler.current(newItem);
        }

        return newItem;
    }

    async function startListenClipboard() {
        const unlistenClipboard = await startListening();
        const unlistenSomethingUpdated = await onSomethingUpdate(async () => {
            if (clipboardChangeHandler.current) {
                const newItem = await insertCopyItem();
                clipboardChangeHandler.current(newItem);
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
        startListenClipboard();
        return () => {
            stopListenClipboard();
        };
    }, []);

    return {
        showPrimaryWindow,
        hidePrimaryWindow,
        showSettingsWindow,
        insertCopyItem,
        reuseCopyItem,
        searchCopyItems,
        showAboutWindow,
    };
}
