import { useEffect, useState, useRef } from 'react';
import { Toolbar } from './toolbar/Toolbar';
import { ItemsList } from './itemsList/ItemsList';
import { CopyItem, CopyCategory, AppError, SearchResult } from '../common/contract';
import { ItemPreview } from './itemPreview/ItemPreview';
import { useDebouncedCallback } from '../common/useDebouncedCallback';
import { useListenClipboard } from './useListenClipboard';
import { error as logError } from 'tauri-plugin-log-api';
import { SearchContext, escapeSearch } from './SearchContext';
import { useSubscribeEvent, emitEvent, ITEMS_CHANGED, WINDOW_HIDDEN_EVENT, HIGHLIGHT_REPORT_BUG } from '../common/events';
import { Container } from './container/Container';
import { hidePrimaryWindow, searchCopyItems, showAboutWindow, showSettingsWindow } from '../common/commands';

const initialQuery = '';
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
    
    useSubscribeEvent<string>(ITEMS_CHANGED, () => search(query, categories));
    useSubscribeEvent<string>(WINDOW_HIDDEN_EVENT, () => {
        // to make refresh in invisible mode
        setSelectedIndex(0);
        toolbarChanged(initialQuery, initialCategories);
    });
    const reuseItemWithoutLoop = useListenClipboard(newItem => applyNewActiveItem(newItem));

    useEffect(() => {
        search(query, categories);
    }, [query, categories]);

    const delayedResetNewlyActive = useDebouncedCallback(() => {
        setNewlyActivedId(null);
    }, 250);

    function toolbarChanged(newQuery: string | null, newCategories: CopyCategory[]) {
        setQuery(newQuery);
        setCategories(newCategories);
    }

    async function refreshAndHide() {
        await hidePrimaryWindow();
    }

    async function search(searchQuery: string | null, searchCategories: CopyCategory[]) {
        setLoading(true);
        setError(null);
        setPage(0);
        setSelectedIndex(0);
        setResult({ items: [], totalNumber: 0 });
        try {
            const resultItems = await searchCopyItems(searchQuery, 0, pageSize, searchCategories);
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
            const resultItems = await searchCopyItems(query, page + 1, pageSize, categories);
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
        if (index >= 0 && index < result.items.length) {
            setSelectedIndex(index);
        }
    }
    async function activate(index: number) {
        if (index >= 0 && index < result.items.length) {
            const oldItem = result.items[index];
            await reuseItemWithoutLoop(oldItem.id);
        }
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

    async function reportIssue() {
        await showAboutWindow();
        await emitEvent(HIGHLIGHT_REPORT_BUG, 'Highlight bug report');
    }
    
    return (
        <Container 
            onHide={refreshAndHide}
            selectedIndex={selectedIndex}
            onSelect={select}
            onActivate={activate}>
            <Toolbar
                query={query}
                categories={categories}
                possibleCategories={possibleCategories}
                onChange={toolbarChanged}
                onClose={hidePrimaryWindow}
                onSettings={showSettingsWindow}
                onReportIssue={reportIssue}
            />
            <SearchContext.Provider value={escapeSearch(query)} >
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
            </SearchContext.Provider>
        </Container>
    );
}

export default App;
