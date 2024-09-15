import { invoke } from '@tauri-apps/api/tauri';
import { AboutData, EventPayload } from '../api';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useRef } from 'react';
import { HIGHLIGHT_REPORT_BUG } from '../events';

async function getAboutData() {
    return await invoke<AboutData>('get_about_data');
}

async function open(target: string) {
    return await invoke<void>('open', { target: target });
}

async function hideAboutWindow() {
    await invoke<void>('hide_about_window');
}

type HighlightReportBugHandler = () => void;

export function useBackend(onHighlightReportBug: HighlightReportBugHandler) {
    const highlightReportBugHandler = useRef<HighlightReportBugHandler>();
    highlightReportBugHandler.current = onHighlightReportBug;

    useEffect(() => {
        async function subscribe() {
            const unlisten = await listen<EventPayload<string>>(HIGHLIGHT_REPORT_BUG, () => {
                if (highlightReportBugHandler.current) {
                    highlightReportBugHandler.current();
                }
            });
            return unlisten;
        }

        const promise = subscribe();
        return () => {
            promise.then((c) => c());
        };
    }, []);

    return {
        getAboutData,
        open,
        hideAboutWindow,
    };
}
