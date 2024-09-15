import { Button } from 'primereact/button';
import { InputText } from 'primereact/inputtext';
import { ComponentProps, useEffect, useRef, useState } from 'react';
import { useDebouncedCallback } from '../../common/useDebouncedCallback';
import { CopyCategory } from '../../api';
import { Checkbox } from 'primereact/checkbox';
import { Image } from 'primereact/image';
import bugImage from '../../assets/bug.png';

import styles from './Toolbar.module.css';

interface ToolbarProps extends Omit<ComponentProps<'div'>, 'onChange'> {
    initialQuery: string | null;
    initialCategories: CopyCategory[];
    possibleCategories: CopyCategory[];
    onChange?: (query: string | null, categories: CopyCategory[]) => void;
    onSettings?: () => void;
    onReportIssue?: () => void;
    onClose?: () => void;
}

function getCategoriesText(categoriesNumber: number, possibleCategories: CopyCategory[]) {
    if (categoriesNumber === (1 << possibleCategories.length) - 1) {
        return 'All';
    }
    if (categoriesNumber == 0) {
        return 'None';
    }

    return toCategoriesArray(categoriesNumber, possibleCategories)
        .map((c) => c.charAt(0))
        .sort((a, b) => a.localeCompare(b))
        .join(',');
}

function toCategoriesNumber(initialCategories: CopyCategory[], possibleCategories: CopyCategory[]) {
    let categories = 0;
    for (let i = 0; i < possibleCategories.length; i++) {
        if (initialCategories.indexOf(possibleCategories[i]) !== -1) {
            categories = categories | (1 << i);
        }
    }
    return categories;
}

function toCategoriesArray(categoriesNumber: number, possibleCategories: CopyCategory[]) {
    let selectedCategories: CopyCategory[] = [];
    let index = 0;
    while (categoriesNumber !== 0) {
        if (categoriesNumber % 2 === 1) {
            selectedCategories.push(possibleCategories[index]);
        }
        index++;
        categoriesNumber = categoriesNumber >> 1;
    }

    return selectedCategories;
}

export function Toolbar({
    onChange,
    onSettings,
    onReportIssue,
    onClose,
    initialQuery,
    initialCategories,
    possibleCategories,
}: ToolbarProps) {
    const [inputValue, setInputValue] = useState<string | undefined>(initialQuery || '');
    const [categoriesNumber, setCategoriesNumber] = useState<number>(toCategoriesNumber(initialCategories, possibleCategories));
    const [categoriesText, setCategoriesText] = useState<string>(getCategoriesText(categoriesNumber, possibleCategories));
    const [isFilterVisible, setIsFilterVisible] = useState<boolean>(false);
    const filterPopupRef = useRef<HTMLDivElement>(null);
    const filterButtonRef = useRef<Button>(null);

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

    const onInputValueChange = useDebouncedCallback((query: string | undefined) => {
        somethingChanged(query, categoriesNumber);
    }, 300);

    function somethingChanged(query: string | undefined, categoriesNumber: number) {
        console.log(toCategoriesArray(categoriesNumber, possibleCategories).join(', '));
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
