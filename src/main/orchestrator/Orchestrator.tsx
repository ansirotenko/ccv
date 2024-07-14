import { useEffect, useState } from 'react';
import { Toolbar } from '../toolbar/Toolbar';
import { CopyItemsList } from '../copyItemsList/CopyItemsList';
import { CopyItem, AppError } from '../api';
import { invoke } from '@tauri-apps/api/tauri';
import { useListenGlobalEvents } from './useListenGlobalEvents';
import { useListenClipboardEvents } from './useListenClipboardEvents';

import styles from './Orchestrator.module.css';

const initialSearchQuery = null;

export function Orchestrator() {
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<AppError | null>(null);
    const [items, setItems] = useState<CopyItem[]>([]);

    useListenGlobalEvents();
    useListenClipboardEvents((patch) => {
        setItems(patch.newItem 
            ? [patch.newItem, ...items.filter(i => !patch.removeIds.includes(i.id))]
            : items.filter(i => !patch.removeIds.includes(i.id))
        );
    });

    async function search(query: string | null) {
        setLoading(true);
        setError(null);
        try {
            //const items =  await invoke<CopyItem[]>('search_copy_items', { query: query || '' });
            const items: CopyItem[] = [
                {id: "1", displayName: "dsiplay name 1"},
                {id: "2", displayName: "dsiplay name 2"},
                {id: "3", displayName: "dsiplay name 3"},
                {id: "4", displayName: "dsiplay name 4"},
                {id: "5", displayName: "dsiplay name 5"},
            ];
            setItems(items);
        } catch (e) {
            setError({message: 'Failed to fetch clipboard history' });
        } finally {
            setLoading(false);
        }
    }

    useEffect(() => {
        search(initialSearchQuery);
    }, []);

    async function hideWindow() {
        await invoke<void>('hide_window');
    }

    return (
        <div className={`container ${styles.container}`}>
            <Toolbar
                value={initialSearchQuery}
                onChange={(q) => {
                    search(q);
                }}
                onClose={() => {
                    hideWindow()
                }}
                onSettings={() => {
                    //TODO
                }}
            />
            <CopyItemsList loading={loading} error={error} items={items} onSelect={(item) => {
                // TODO complete here
                console.log(`selected ${item.displayName}`);
            }} />
        </div>
    );
}
