import { Button } from 'primereact/button';
import { InputText } from 'primereact/inputtext';
import { ComponentProps, useEffect, useRef, useState } from 'react';
import { useDebouncedCallback } from '../../common/useDebouncedCallback';
import { CopyCategory } from '../../common/contract';
import { Checkbox } from 'primereact/checkbox';
import { useSubscribeEvent, WINDOW_SHOWN_EVENT, WINDOW_HIDDEN_EVENT } from '../../common/events';
import { getCategoriesText, toCategoriesArray, toCategoriesNumber } from './categoryHelper';
import { defaultCategories, possibleCategories, defaultQuery } from '../SearchContext';
import bugImage from '../../assets/bug.png';

import styles from './Toolbar.module.css';

interface ToolbarProps extends Omit<ComponentProps<'div'>, 'onChange'> {
    onChange?: (query: string | null, categories: CopyCategory[]) => void;
    onSettings?: () => void;
    onReportIssue?: () => void;
    onClose?: () => void;
}

const defaultCategoriesNumber = toCategoriesNumber(defaultCategories, possibleCategories);
const defaultCategoriesText = getCategoriesText(defaultCategoriesNumber, possibleCategories);

export function Toolbar({ onChange, onSettings, onReportIssue, onClose }: ToolbarProps) {
    const [inputValue, setInputValue] = useState<string>(defaultQuery);
    const [categoriesNumber, setCategoriesNumber] = useState<number>(defaultCategoriesNumber);
    const [categoriesText, setCategoriesText] = useState<string>(defaultCategoriesText);
    const [isFilterVisible, setIsFilterVisible] = useState<boolean>(false);
    const filterPopupRef = useRef<HTMLDivElement>(null);
    const filterButtonRef = useRef<Button>(null);

    const [counter, setCounter] = useState<number>(0);

    useSubscribeEvent<string>(WINDOW_SHOWN_EVENT, () => {
        setCounter((c) => c + 1); // to provoke rerender, that would focus input
    });
    useSubscribeEvent<string>(WINDOW_HIDDEN_EVENT, () => {
        setCategoriesNumber(defaultCategoriesNumber);
        setCategoriesText(defaultCategoriesText);
        setInputValue(defaultQuery);
        somethingChanged(defaultQuery, defaultCategoriesNumber);
    });

    useEffect(() => {
        function handleClickOutside(event: MouseEvent) {
            const target = event.target as Node;
            if (
                filterPopupRef.current &&
                !filterPopupRef.current.contains(target) &&
                filterButtonRef.current &&
                !(filterButtonRef.current as any).contains(target)
            ) {
                setIsFilterVisible(false);
            }
        }
        document.addEventListener('mousedown', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, [filterPopupRef, filterButtonRef]);

    const onInputValueChange = useDebouncedCallback((query: string) => {
        somethingChanged(query, categoriesNumber);
    }, 200);

    function somethingChanged(query: string, categoriesNumber: number) {
        if (onChange != null) {
            onChange(query || null, toCategoriesArray(categoriesNumber, possibleCategories));
        }
    }

    return (
        <div className={`${styles.container}`}>
            <i className={`pi pi-search ${styles.searchIcon}`} data-tauri-drag-region />
            <span className={`p-inputgroup ${styles.searchInputGroup}`}>
                <InputText
                    className={`${styles.searchInput}`}
                    data-counter={counter}
                    placeholder="Search..."
                    value={inputValue}
                    ref={(input) => {input && input.focus();}}
                    onChange={(e) => {
                        setInputValue(e.target.value);
                        onInputValueChange(e.target.value);
                    }}
                />
                {inputValue && (
                    <i
                        className={`pi pi-trash ${styles.clearSearch}`}
                        onClick={() => {
                            const newinputValue = '';
                            setInputValue(newinputValue);
                            somethingChanged(newinputValue, categoriesNumber);
                        }}
                    ></i>
                )}
            </span>
            <div ref={filterPopupRef} className={`${styles.filterPopup} ${isFilterVisible ? styles.visibleFilterPopup : ''}`}>
                <div
                    className={styles.filterItem}
                    onClick={() => {
                        const newCategoriesNumber = (1 << possibleCategories.length) - 1;
                        setCategoriesNumber(newCategoriesNumber);
                        setCategoriesText(getCategoriesText(newCategoriesNumber, possibleCategories));
                        somethingChanged(inputValue, newCategoriesNumber);
                    }}
                >
                    <Checkbox checked={categoriesNumber + 1 === 1 << possibleCategories.length} />
                    <label htmlFor="All" className={styles.filterItemLabel}>
                        All
                    </label>
                </div>
                {possibleCategories.map((category, index) => {
                    return (
                        <div
                            key={category}
                            className={styles.filterItem}
                            onClick={() => {
                                const hasCategory = (categoriesNumber & (1 << index)) === 0;
                                const newCategoriesNumber = hasCategory
                                    ? categoriesNumber | (1 << index)
                                    : categoriesNumber & (((1 << possibleCategories.length) - 1) ^ (1 << index));
                                setCategoriesNumber(newCategoriesNumber);
                                setCategoriesText(getCategoriesText(newCategoriesNumber, possibleCategories));
                                somethingChanged(inputValue, newCategoriesNumber);
                            }}
                        >
                            <Checkbox checked={(categoriesNumber & (1 << index)) !== 0} />
                            <label htmlFor={category} className={styles.filterItemLabel}>
                                {category}
                            </label>
                        </div>
                    );
                })}
            </div>
            <Button
                ref={filterButtonRef}
                tooltip="Filter"
                tooltipOptions={{ position: 'left', style: { fontSize: 13 } }}
                className={styles.buttons}
                onClick={() => {
                    setIsFilterVisible(!isFilterVisible);
                }}
            >
                {categoriesText}
            </Button>
            <Button
                tooltip="Report issue"
                tooltipOptions={{ position: 'left', style: { fontSize: 13 } }}
                className={`${styles.buttons}`}
                onClick={() => {
                    if (onReportIssue != null) {
                        onReportIssue();
                    }
                }}
            >
                <img src={bugImage} />
            </Button>
            <Button
                tooltip="Settings"
                tooltipOptions={{ position: 'left', style: { fontSize: 13 } }}
                className={`pi pi-cog ${styles.buttons}`}
                onClick={() => {
                    if (onSettings != null) {
                        onSettings();
                    }
                }}
            />
            <Button
                tooltip="Close"
                tooltipOptions={{ position: 'left', style: { fontSize: 13 } }}
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
