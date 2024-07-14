import { ComponentProps } from 'react';
import { CopyItem, AppError } from '../api';
import { ProgressSpinner } from 'primereact/progressspinner';
import { Message } from 'primereact/message';

import styles from './CopyItemsList.module.css';

interface CopyItemsListProps extends Omit<ComponentProps<'div'>, 'onSelect'> {
    loading: boolean;
    error: AppError | null,
    items: CopyItem[],
    onSelect: (item: CopyItem) => void
}

export function CopyItemsList({ loading, error, items, onSelect}: CopyItemsListProps) {

    if (loading) {
        return <ProgressSpinner className={styles.loading} />;
    }
    if (error) {
        return <Message severity="error" className={styles.failed} text={error.message} />;
    }
    if (!items.length) {
        return <em className={styles.empty}>Nothing was found</em>;
    }

    return (
        <ul>
            {items.map((item) => (
                <li key={item.id} onDoubleClick={() => {
                    if (onSelect) {
                        onSelect(item);
                    }
                }}>
                    {`id: ${item.id}, displayName: ${item.displayName}`}
                </li>
            ))}
        </ul>
    );
}
