import { useEffect, useState, KeyboardEvent, useRef } from 'react';
import { Toolbar } from './toolbar/Toolbar';
import { ItemsList } from './itemsList/ItemsList';
import { CopyItem, CopyCategory, AppError, SearchResult } from '../api';
import { ItemPreview } from './itemPreview/ItemPreview';
import { useDebouncedCallback } from '../common/useDebouncedCallback';
import { useBackend } from './useBackend';
import { error as logError } from 'tauri-plugin-log-api';
import { highlightSearchTerm } from 'highlight-search-term';

import styles from './App.module.css';

const initialQuery = null;
const possibleCategories: CopyCategory[] = ['Files', 'Html', 'Image', 'Rtf', 'Text'];
const initialCategories: CopyCategory[] = possibleCategories;
const pageSize = 100;

function App() {
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<AppError | null>(null);
    const [page, setPage] = useState<number>(0);
    const [result, setResult] = useState<SearchResult>({ items: [], totalNumber: 0 });
    const [selectedIndex, setSelectedIndex] = useState(0);
    const [newlyActivedId, setNewlyActivedId] = useState<string | null>(null);
    const [query, setQuery] = useState<string | null>(initialQuery);
    const [categories, setCategories] = useState<CopyCategory[]>(initialCategories);
    const containerRef = useRef<HTMLDivElement>(null);

    const backend = useBackend(
        (newItem) => {
            applyNewActiveItem(newItem);
        },
        () => {
            search(query, categories);
        },
    );
    useEffect(() => {
        containerRef.current?.focus();
        onToolbarChange(initialQuery, initialCategories);
    }, []);

    useEffect(() => {
        highlightSearchTerm({ search: query || '', selector: `.highlight` });
    }, [query, result.items, result.items[selectedIndex]]);

    const delayedResetNewlyActive = useDebouncedCallback(() => {
        setNewlyActivedId(null);
    }, 250);

    async function onToolbarChange(newQuery: string | null, newCategories: CopyCategory[]) {
        setQuery(newQuery);
        setCategories(newCategories);
        await search(newQuery, newCategories);
    }

    async function search(searchQuery: string | null, searchCategories: CopyCategory[]) {
        setLoading(true);
        setError(null);
        setPage(0);
        setSelectedIndex(0);
        setResult({ items: [], totalNumber: 0 });
        try {
            const resultItems = await backend.searchCopyItems(searchQuery, 0, pageSize, searchCategories);
            setResult(resultItems);
        } catch (e) {
            logError((e as AppError).message);
            setError({ message: (e as AppError).message });
        } finally {
            setLoading(false);
        }
    }

    async function loadMore() {
        setLoading(true);
        try {
            const resultItems = await backend.searchCopyItems(query, page + 1, pageSize, categories);
            setResult({
                items: [...result.items, ...resultItems.items],
                totalNumber: resultItems.totalNumber,
            });
            setPage(page + 1);
            containerRef.current?.focus();
        } catch (e) {
            logError((e as AppError).message);
            setError({ message: (e as AppError).message });
        } finally {
            setLoading(false);
        }
    }

    function select(index: number) {
        setSelectedIndex(index);
    }
    async function activate(item: CopyItem) {
        const newItem = await backend.reuseCopyItem(item.id);
        applyNewActiveItem(newItem);
    }
    async function applyNewActiveItem(newItem: CopyItem) {
        if (categories.indexOf(newItem.value.category) === -1) {
            return;
        }
        const newItemIndex = result.items.findIndex((x) => x.id === newItem.id);
        if (newItemIndex === -1) {
            setResult({
                items: [newItem, ...result.items],
                totalNumber: result.totalNumber + 1,
            });
            setSelectedIndex(selectedIndex + 1);
        } else {
            setResult({
                items: [newItem, ...result.items.filter((x) => x.id !== newItem.id)],
                totalNumber: result.totalNumber,
            });
            if (newItemIndex === selectedIndex) {
                setSelectedIndex(0);
            } else {
                if (newItemIndex >= selectedIndex) {
                    setSelectedIndex(selectedIndex + 1);
                }
            }
        }
        setNewlyActivedId(newItem.id);
        delayedResetNewlyActive();
    }
    const keyDown = (event: KeyboardEvent<HTMLDivElement>) => {
        if (event.key === 'Enter') {
            activate(result.items[selectedIndex]);
            return;
        }
        if (event.key === 'ArrowUp') {
            if (selectedIndex > 0) {
                setSelectedIndex(selectedIndex - 1);
            }
            return;
        }
        if (event.key === 'ArrowDown') {
            if (selectedIndex < result.items.length - 1) {
                setSelectedIndex(selectedIndex + 1);
            }
            return;
        }
        if (event.ctrlKey && event.key >= '1' && event.key <= '9') {
            const index = parseInt(event.key) - 1;
            if (index < result.items.length) {
                activate(result.items[index]);
            }
            return;
        }
    };

    return (
        <div className={styles.container} onKeyDown={keyDown} tabIndex={0} ref={containerRef}>
            <Toolbar
                initialQuery={initialQuery}
                initialCategories={initialCategories}
                possibleCategories={possibleCategories}
                onChange={onToolbarChange}
                onClose={backend.hideMainWindow}
                onSettings={backend.showSettingsWindow}
                onReportIssue={backend.showAboutWindow}
            />
            <ItemsList
                loading={loading}
                error={error}
                result={result}
                selectedIndex={selectedIndex}
                newlyActivedId={newlyActivedId}
                onSelect={select}
                onActivate={activate}
                onLoadMore={loadMore}
            />
            <ItemPreview item={result.items[selectedIndex]} />
        </div>
    );
}

export default App;
