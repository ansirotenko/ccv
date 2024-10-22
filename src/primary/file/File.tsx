import { ComponentProps, useContext } from 'react';
import { FileInfo } from '../../common/contract';
import { SearchContext, textToHighlightedHtml } from '../SearchContext';

import styles from './File.module.css';

interface FileProps extends ComponentProps<'div'> {
    file: FileInfo;
}

export function File({ file }: FileProps) {
    const search = useContext(SearchContext);
    return (
        <div className={`file ${styles.container}`}>
            <div className="iconus">
                {file.iconBase64 && <img src={`data:image/png;base64, ${file.iconBase64}`} />}
                {!file.iconBase64 && !file.isDirectory && <i className="pi pi-file"></i>}
                {!file.iconBase64 && file.isDirectory && <i className="pi pi-folder"></i>}
            </div>
            <em dangerouslySetInnerHTML={{ __html: textToHighlightedHtml(file.fileName, search) }}></em>
            <em> @ </em>
            <em dangerouslySetInnerHTML={{ __html: textToHighlightedHtml(file.fullPath, search) }}></em>
        </div>
    );
}
