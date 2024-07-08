import { Button } from 'primereact/button';
import { InputText } from 'primereact/inputtext';
import { ComponentProps, useState } from 'react';
import styles from './Toolbar.module.css';
import { useDebouncedCallback } from '../utils/useDebouncedCallback';

interface ToolbarProps extends Omit<ComponentProps<'div'>, 'onChange'> {
    value: string | null;
    onChange?: (query: string | null) => void;
    onSettings?: () => void;
    onClose?: () => void;
}

export function Toolbar({ onChange, onSettings, onClose, value }: ToolbarProps) {
    const [inputValue, setInputValue] = useState<string | undefined>(value || '');

    const onInputValueChange = useDebouncedCallback((query: string | undefined) => {
        if (onChange != null) {
            onChange(query || null);
        }
    }, 200);

    return (
        <div className="p-inputgroup">
            <Button
                className={`pi pi-search ${styles.buttons}`}
                disabled={true}
            />
            <span className={`p-inputgroup ${styles.searchInputGroup}`}>
                <InputText
                    className={`${styles.searchInput}`}
                    placeholder="Search..."
                    value={inputValue}
                    onChange={(e) => {
                        setInputValue(e.target.value);
                        onInputValueChange(e.target.value);
                    }}
                />
                {inputValue && (
                    <i
                        className={`pi pi-trash ${styles.clearSearch}`}
                        onClick={() => {
                            setInputValue('');
                            if (onChange != null) {
                                onChange(null);
                            }
                        }}
                    ></i>
                )}
            </span>
            <Button
                className={`pi pi-cog ${styles.buttons}`}
                onClick={() => {
                    if (onSettings != null) {
                        onSettings();
                    }
                }}
            />
            <Button
                className={`pi pi-times ${styles.buttons}`}
                onClick={() => {
                    if (onClose != null) {
                        onClose();
                    }
                }}
            />
        </div>
    );
}
