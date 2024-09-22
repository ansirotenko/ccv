import { ComponentProps, useContext } from 'react';
import { FileInfo } from '../../api';
import { SearchContext, textToHighlightedHtml } from '../SearchContext';

import styles from './FilePreview.module.css';

interface FilePreviewProps extends ComponentProps<'div'> {
    file: FileInfo;
}

export function FilePreview({ file }: FilePreviewProps) {
    const search = useContext(SearchContext);
    return (
        <div className={styles.container}>
            {file.iconBase64 && <img className={styles.image} src={`data:image/png;base64, ${file.iconBase64}`} height="13" />}
            {!file.iconBase64 && !file.isDirectory && <i className={`pi pi-file ${styles.icon}`}></i>}
            {!file.iconBase64 && file.isDirectory && <i className={`pi pi-folder ${styles.icon}`}></i>}
            <em dangerouslySetInnerHTML={{ __html: textToHighlightedHtml(file.fileName, search) }}></em>
            <em> @ </em>
            <em dangerouslySetInnerHTML={{ __html: textToHighlightedHtml(file.fullPath, search) }}></em>
        </div>
    );
}
