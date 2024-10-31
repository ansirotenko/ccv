import { ComponentProps } from 'react';
import { AppError, SearchResult } from '../../common/contract';
import { Message } from 'primereact/message';
import { Item } from '../item/Item';

import styles from './ItemsList.module.css';

interface ItemsListProps extends Omit<ComponentProps<'div'>, 'onSelect'> {
    error: AppError | null;
    result: SearchResult;
    selectedIndex: number;
    newlyActivedId: string | null;
    onSelect: (index: number) => void;
    onActivate: (index: number) => void;
    onLoadMore: () => void;
}

export function ItemsList({ error, result, selectedIndex, newlyActivedId, onSelect, onActivate, onLoadMore }: ItemsListProps) {
    if (error) {
        return <Message severity="error" className={styles.failed} text={error.message} />;
    }

    if (!result.items.length) {
        return <em className={styles.empty}>Nothing was found</em>;
    }

    return (
        <div className={styles.items}>
            {result.items.map((item, i) => (
                <Item
                    key={item.id}
                    item={item}
                    selectedIndex={selectedIndex}
                    newlyActivedId={newlyActivedId}
                    index={i}
                    onSelect={onSelect}
                    onActivate={onActivate}
                />
            ))}
            {result.totalNumber > result.items.length && (
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
        </div>
    );
}
