import { ComponentProps, useEffect, useRef } from 'react';
import { CopyItem } from '../../api';
import { Message } from 'primereact/message';
import { Image } from 'primereact/image';
import DOMPurify from 'dompurify';
import { Tooltip } from 'primereact/tooltip';

import styles from './Item.module.css';

interface ItemProps extends Omit<ComponentProps<'div'>, 'onSelect'> {
    item: CopyItem;
    index: number;
    selectedIndex: number;
    newlyActivedId: string | null;
    onSelect: (index: number) => void;
    onActivate: (item: CopyItem) => void;
}

export function Item({ item, index, selectedIndex, newlyActivedId, onSelect, onActivate }: ItemProps) {
    const ref = useRef<HTMLDivElement>(null);
    useEffect(() => {
        if (index === selectedIndex) {
            ref.current?.scrollIntoView({
                behavior: 'instant',
                block: 'nearest',
            });
        }
    }, [selectedIndex, index]);

    function content() {
        if (item.value.image) {
            return (
                <div className={styles.imageContent}>
                    <Image src={`data:image/png;base64, ${item.value.image}`} height="50" />
                </div>
            );
        }

        if (item.value.files) {
            const fileClass = item.value.files.length === 1 ? styles.singleLineFile : styles.multiLineFiles;
            return (
                <div className={`highlight ${fileClass}`}>
                    {item.value.files.map((f) => (
                        <div key={f.fullPath}>
                            <em>{`${f.fileName} @ ${f.directoryPath}`}</em>
                        </div>
                    ))}
                </div>
            );
        }

        if (item.value.text) {
            const textClass = item.value.text.indexOf('\n') === -1 ? styles.singleLineText : styles.multiLineText;
            return <div className={`highlight ${textClass}`}>{item.value.text}</div>;
        }

        if (item.value.rtf) {
            return (
                <div
                    className={`highlight ${styles.htmlContent}`}
                    dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(item.value.rtf) }}
                />
            );
        }

        if (item.value.html) {
            return (
                <div
                    className={`highlight ${styles.htmlContent}`}
                    dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(item.value.html) }}
                />
            );
        }

        return <Message severity="error" className={styles.failed} text={`Copy item has empty content or format is unknown`} />;
    }

    return (
        <div
            ref={ref}
            className={`${styles.container} ${selectedIndex === index ? styles.selected : ''} ${newlyActivedId === item.id ? styles.newlyActive : ''}`}
            onClick={() => {
                if (onSelect) {
                    onSelect(index);
                }
            }}
            onDoubleClick={() => {
                if (onActivate) {
                    onActivate(item);
                }
            }}
        >
            {index < 9 && (
                <Tooltip target={`.item-${index}`}>
                    <div className={styles.tooltip} data-pr-tooltip={`hint${index}`} data-pr-position="right">
                        {`Press Ctrl+${index + 1} to reuse item`}
                    </div>
                </Tooltip>
            )}
            <div className={`${styles.index} item-${index}`}>{index + 1}</div>
            {content()}
        </div>
    );
}
