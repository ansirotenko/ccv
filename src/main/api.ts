export interface CopyItem {
    id: string;
    displayName: string;
}

export interface CopyItemsPatch {
    newItem: CopyItem | null;
    removeIds: string[];
}

export type AppError = {
    message: string;
}