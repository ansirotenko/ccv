import { useEffect, useRef } from 'react';
import { onSomethingUpdate, startListening } from 'tauri-plugin-clipboard-api';
import { CopyItem } from '../common/contract';
import { insertCopyItem, reuseCopyItem } from '../common/commands';

type ClipboardChangedHandler = (patch: CopyItem) => void;
type UnlistendClipboardFunc = () => Promise<void>;

export function useListenClipboard(onClipboardChanged: ClipboardChangedHandler) {
    const clipboardChangeHandler = useRef<ClipboardChangedHandler>();
    clipboardChangeHandler.current = onClipboardChanged;

    const unlistenSomethingUpdatedRef = useRef<UnlistendClipboardFunc>();

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

    return async function (itemId: string) {
        await stopListenClipboard();
        const newItem = await reuseCopyItem(itemId);
        await startListenClipboard();

        if (clipboardChangeHandler.current) {
            clipboardChangeHandler.current(newItem);
        }

        return newItem;
    };
}
