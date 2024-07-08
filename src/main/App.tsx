import { useEffect, useState } from 'react';
import { Toolbar } from './toolbar/Toolbar';
import { searchCopyItems, CopyItem, listenGlobalEvents } from './api';
import { ProgressSpinner } from 'primereact/progressspinner';
import { Message } from 'primereact/message';

import styles from './App.module.css';

const initialSearchQuery = null;

function App() {
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [items, setItems] = useState<CopyItem[]>([]);

    async function search(query: string | null) {
        setLoading(true);
        setError(null);
        try {
            const items = await searchCopyItems(query);
            setItems(items);
        } catch (e) {
            setError('Failed to fetch clipboard history');
        } finally {
            setLoading(false);
        }
    }

    useEffect(() => {
        listenGlobalEvents();
        search(initialSearchQuery);
    }, []);

    function content() {
        if (loading) {
            return <ProgressSpinner className={styles.loading} />;
        }
        if (error) {
            return <Message severity="error" className={styles.failed} text={error} />;
        }
        if (!items.length) {
            return <em className={styles.empty}>Nothing was found</em>;
        }

        return (
            <ul>
                {items.map((item) => (
                    <li key={item.id}>{`id: ${item.id}, displayName: ${item.displayName}`}</li>
                ))}
            </ul>
        );
    }

    return (
        <div className={`container ${styles.container}`}>
            <Toolbar
                value={initialSearchQuery}
                onChange={(q) => {
                    search(q);
                }}
                onClose={() => {
                    //TODO
                }}
                onSettings={() => {
                    //TODO
                }}
            />
            {content()}
        </div>
    );
}

export default App;
