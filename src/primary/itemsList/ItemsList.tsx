import { ComponentProps, useEffect, useRef } from 'react';
import { AppError, SearchResult } from '../../common/contract';
import { ProgressSpinner } from 'primereact/progressspinner';
import { Message } from 'primereact/message';
import { Item } from '../item/Item';

import styles from './ItemsList.module.css';

interface ItemsListProps extends Omit<ComponentProps<'div'>, 'onSelect'> {
    loading: boolean;
    error: AppError | null;
    result: SearchResult;
    selectedIndex: number;
    newlyActivedId: string | null;
    onSelect: (index: number) => void;
    onActivate: (index: number) => void;
    onLoadMore: () => void;
}

export function ItemsList({ loading, error, result, selectedIndex, newlyActivedId, onSelect, onActivate, onLoadMore }: ItemsListProps) {
    const loadingMoreRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (loadingMoreRef.current) {
            loadingMoreRef.current.scrollIntoView();
        }
    }, [error, result, loading]);

    if (error) {
        return <Message severity="error" className={styles.failed} text={error.message} />;
    }

    if (!result.items.length) {
        if (loading) {
            return <ProgressSpinner className={styles.loading} />;
        } else {
            return <em className={styles.empty}>Nothing was found</em>;
        }
    }

    return (
        <div className={styles.items}>
            {result.items.map((item, i) => (
                <Item
                    key={i}
                    item={item}
                    selectedIndex={selectedIndex}
                    newlyActivedId={newlyActivedId}
                    index={i}
                    onSelect={onSelect}
                    onActivate={onActivate}
                />
            ))}
            {!loading && result.totalNumber > result.items.length && (
                <a
                    className={styles.hasMore}
                    href="#"
                    onClick={() => {
                        if (onLoadMore) {
                            onLoadMore();
                        }
                    }}
                >
                    {`and ${result.totalNumber - result.items.length} more...`}
                </a>
            )}
            {loading && (
                <>
                    <ProgressSpinner className={styles.loading} />
                    <div ref={loadingMoreRef}></div>
                </>
            )}
        </div>
    );
}
