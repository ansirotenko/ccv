export type AppError = {
    message: string;
};

export interface SearchResult {
    items: CopyItem[];
    totalNumber: number;
}

export interface CopyItem {
    id: string;
    created: Date;
    lastReuse: Date;
    value: CopyItemValue;
}

export interface CopyItemValue {
    text?: string;
    html?: string;
    rtf?: string;
    image?: string;
    files?: FileInfo[];
    category: CopyCategory;
}

export type CopyCategory = 'Text' | 'Html' | 'Rtf' | 'Image' | 'Files' | 'Unknown';

export interface FileInfo {
    fullPath: string;
    directoryPath: string;
    fileName: string;
    iconBase64?: string;
    isDirectory: boolean;
}

export interface Shortcut {
    altKey: boolean;
    ctrlKey: boolean;
    code: string | undefined;
    metaKey: boolean;
    shiftKey: boolean;
}

export interface AllShortcuts {
    openCcv: Shortcut;
}

export type Theme = 'Light' | 'Dark';

export interface Settings {
    allShortcuts: AllShortcuts;
    theme: Theme;
}

export interface AboutData {
    version: string;
    authors: string;
    homepage: string;
    appDirectory: string | null;
    appDataDirectory: string | null;
    appLogsDirectory: string | null;
    text: string;
}

export interface EventPayload<T> {
    data: T;
}

export interface MainShortcutPressedPayload {
    changedFromHiddenToVisile: boolean;
}