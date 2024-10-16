import { ComponentProps, useContext } from 'react';
import { FileInfo } from '../../common/contract';
import { SearchContext, textToHighlightedHtml } from '../SearchContext';

import styles from './FilePreview.module.css';

interface FilePreviewProps extends ComponentProps<'div'> {
    file: FileInfo;
}

export function FilePreview({ file }: FilePreviewProps) {
    const search = useContext(SearchContext);
    return (
        <div className={styles.container}>
            <div className={styles.iconus}>
                {file.iconBase64 && <img src={`data:image/png;base64, ${file.iconBase64}`} height="16" />}
                {!file.iconBase64 && !file.isDirectory && <i className="pi pi-file"></i>}
                {!file.iconBase64 && file.isDirectory && <i className="pi pi-folder"></i>}
            </div>
            <em dangerouslySetInnerHTML={{ __html: textToHighlightedHtml(file.fileName, search) }}></em>
            <em> @ </em>
            <em dangerouslySetInnerHTML={{ __html: textToHighlightedHtml(file.fullPath, search) }}></em>
        </div>
    );
}
