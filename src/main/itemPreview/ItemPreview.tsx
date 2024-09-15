import { ComponentProps } from 'react';
import { CopyItem } from '../../api';
import { Message } from 'primereact/message';
import { Image } from 'primereact/image';
import { FilePreview } from './FilePreview';
import DOMPurify from 'dompurify';

import styles from './ItemPreview.module.css';
import { RtfPreview } from './RtfPreview';

interface ItemPreviewProps extends ComponentProps<'div'> {
    item: CopyItem;
}

export function ItemPreview({ item }: ItemPreviewProps) {
    function content() {
        if (item.value.image) {
            return (
                <div className={styles.imageContent}>
                    <Image src={`data:image/png;base64, ${item.value.image}`} height="122" preview />
                </div>
            );
        }

        if (item.value.files) {
            return (
                <div className={styles.filesContent}>
                    {item.value.files.map((f) => (
                        <FilePreview key={f.fullPath} file={f} />
                    ))}
                </div>
            );
        }

        if (item.value.rtf) {
            return <RtfPreview rtf={item.value.rtf} />;
        }

        if (item.value.html) {
            return <div className={styles.htmlContent} dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(item.value.html) }} />;
        }

        if (item.value.text) {
            return <div className={styles.textContent}>{item.value.text}</div>;
        }

        return <Message severity="error" className={styles.failed} text={`Copy item has empty content or format is unknown`} />;
    }

    if (!item) {
        return <div className={styles.container}></div>;
    }

    let info = `id: ${item.id}, created: ${item.created.toLocaleString()}`;
    if (item.lastReuse.getTime() !== item.created.getTime()) {
        info += `, last use: ${item.lastReuse.toLocaleString()}`;
    }
    return (
        <div className={`highlight ${styles.container}`}>
            <div className={styles.contentContainer}>{content()}</div>
            <div className={styles.infoContainer}>{info}</div>
        </div>
    );
}
