import { ComponentProps, useContext } from 'react';
import { CopyItem } from '../../common/contract';
import { Message } from 'primereact/message';
import { Image } from 'primereact/image';
import { File } from '../file/File';
import DOMPurify from 'dompurify';
import { RtfPreview } from './RtfPreview';
import { SearchContext, textToHighlightedHtml, htmlToHighlightedHtml } from '../SearchContext';

import styles from './ItemPreview.module.css';

interface ItemPreviewProps extends ComponentProps<'div'> {
    item: CopyItem;
}

export function ItemPreview({ item }: ItemPreviewProps) {
    const search = useContext(SearchContext);

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
                    {item.value.files.map((f) => (<File key={f.fullPath} file={f} />))}
                </div>
            );
        }

        if (item.value.rtf) {
            return <RtfPreview rtf={item.value.rtf} />;
        }

        if (item.value.html) {
            return (
                <div
                    className={styles.htmlContent}
                    dangerouslySetInnerHTML={{ __html: htmlToHighlightedHtml(DOMPurify.sanitize(item.value.html), search) }}
                />
            );
        }

        if (item.value.text) {
            return (
                <div
                    className={styles.textContent}
                    dangerouslySetInnerHTML={{ __html: textToHighlightedHtml(item.value.text, search) }}
                ></div>
            );
        }

        return <Message severity="error" className={styles.failed} text={`Copy item has empty content or format is unknown`} />;
    }

    if (!item) {
        return <div className={styles.container}></div>;
    }

    let info = `id: ${item.id}, created: ${item.created.toLocaleString('en-GB')}`;
    if (item.lastReuse.getTime() !== item.created.getTime()) {
        info += `, last use: ${item.lastReuse.toLocaleString('en-GB')}`;
    }
    return (
        <div className={styles.container}>
            <div className={styles.contentContainer}>{content()}</div>
            <div className={styles.infoContainer}>{info}</div>
        </div>
    );
}
