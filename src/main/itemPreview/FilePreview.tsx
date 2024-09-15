import { ComponentProps } from 'react';
import { FileInfo } from '../../api';

import styles from './FilePreview.module.css';

interface FilePreviewProps extends ComponentProps<'div'> {
    file: FileInfo;
}

export function FilePreview({ file }: FilePreviewProps) {
    return (
        <div className={styles.container}>
            {file.iconBase64 && <img className={styles.image} src={`data:image/png;base64, ${file.iconBase64}`} height="13" />}
            {!file.iconBase64 && !file.isDirectory && <i className={`pi pi-file ${styles.icon}`}></i>}
            {!file.iconBase64 && file.isDirectory && <i className={`pi pi-folder ${styles.icon}`}></i>}
            <em>{`${file.fileName} @ ${file.fullPath}`}</em>
        </div>
    );
}
