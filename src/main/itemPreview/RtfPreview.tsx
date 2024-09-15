import { ComponentProps, useEffect, useState } from 'react';
import { Message } from 'primereact/message';
import { ProgressSpinner } from 'primereact/progressspinner';
import DOMPurify from 'dompurify';
import { EMFJS, RTFJS, WMFJS } from 'rtf.js';

import styles from './RtfPreview.module.css';

RTFJS.loggingEnabled(false);
WMFJS.loggingEnabled(false);
EMFJS.loggingEnabled(false);

interface RtfPreviewProps extends ComponentProps<'div'> {
    rtf: string;
}

function stringToArrayBuffer(rtf: string) {
    const buffer = new ArrayBuffer(rtf.length);
    const bufferView = new Uint8Array(buffer);
    for (let i = 0; i < rtf.length; i++) {
        bufferView[i] = rtf.charCodeAt(i);
    }
    return buffer;
}

export function RtfPreview({ rtf }: RtfPreviewProps) {
    const [html, setHtml] = useState<HTMLElement[] | undefined>();
    const [error, setError] = useState<string | null>(null);
    const [loading, setLoading] = useState<boolean>(true);

    useEffect(() => {
        const doc = new RTFJS.Document(stringToArrayBuffer(rtf), {});
        setError(null);
        setLoading(true);
        doc.render()
            .then((newHtml) => {
                setHtml(newHtml);
            })
            .catch((error) => {
                setError(`Unable to parse rtf. ${error}`);
            })
            .finally(() => {
                setLoading(false);
            });
    }, [rtf]);

    if (loading) {
        return <ProgressSpinner className={styles.loading} />;
    }

    if (error) {
        return <Message severity="error" className={styles.failed} text={error} />;
    }

    return (
        <div className={styles.rtfContent}>
            {html && html.map((h, i) => <div key={i} dangerouslySetInnerHTML={{ __html: DOMPurify.sanitize(h) }} />)}
        </div>
    );
}
