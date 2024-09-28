import { EventPayload } from '../common/contract';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef } from 'react';
import { emit } from '@tauri-apps/api/event';

type EventHandler<TPayload> = (payload: TPayload) => void;

export const ITEMS_CHANGED = 'items-changed';
export const SETTINGS_UPDATED = 'settings-updated';
export const HIGHLIGHT_REPORT_BUG = 'highlight-report-bug';
export const MAIN_SHORTCUT_PRESSED_EVENT = 'main-shortcut-pressed';
export const WINDOW_SHOWN_EVENT = 'window-shown';
export const WINDOW_HIDDEN_EVENT = 'window-hidden';

export function useSubscribeEvent<TPayload>(name: string, handler: EventHandler<TPayload>) {
    const handlerRef = useRef<EventHandler<TPayload>>();
    handlerRef.current = handler;

    useEffect(() => {
        async function subscribe() {
            const unlisten = await listen<EventPayload<TPayload>>(name, (event) => {
                if (handlerRef.current) {
                    handlerRef.current(event.payload.data);
                }
            });
            return unlisten;
        }

        const promise = subscribe();
        return () => {
            promise.then((c) => c());
        };
    }, []);
}

export async function emitEvent<TPayload>(name: string, payload: TPayload) {
    await emit(name, { data: payload } as EventPayload<string>);
}
