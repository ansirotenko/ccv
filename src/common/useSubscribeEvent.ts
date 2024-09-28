import { EventPayload } from '../api';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef } from 'react';

type EventHandler<TPayload> = (payload: TPayload) => void;

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