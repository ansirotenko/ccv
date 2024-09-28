import { KeyboardEvent, useRef, ComponentProps, useState, useContext } from 'react';
import { MainShortcutPressedPayload } from '../../api';
import { useSubscribeEvent } from '../../common/useSubscribeEvent';
import { MAIN_SHORTCUT_PRESSED_EVENT, WINDOW_SHOWN_EVENT } from '../../events';
import { hasModifers, matchShortcutModifiers } from '../../common/keyboard';
import SettingsContext from '../../common/SettingsContext';

import styles from './Container.module.css';

interface ContainerProps extends Omit<ComponentProps<'div'>, 'onSelect'> {
    selectedIndex: number;
    onSelect: (index: number) => void;
    onActivate: (index: number) => void;
    onHide: () => void;
}

export function Container({ selectedIndex, onSelect, onActivate, onHide, children}: ContainerProps) {
    const containerRef = useRef<HTMLDivElement>(null);
    const [mainShortcutOn, setMainShortcutOn] = useState<boolean>(false);
    const [mainShortcutCounter, setMainShortcutCounter] = useState<number>(0);
    const settings = useContext(SettingsContext);

    useSubscribeEvent<string>(WINDOW_SHOWN_EVENT, () => {
        containerRef.current?.focus();
    });

    useSubscribeEvent<MainShortcutPressedPayload>(MAIN_SHORTCUT_PRESSED_EVENT, (mainShortcutPressedPayload) => {
        if (mainShortcutPressedPayload.changedFromHiddenToVisile) {
            if (hasModifers(settings.allShortcuts.openCcv)) {
                setMainShortcutOn(true);
                setMainShortcutCounter(0);
            }
        } else {
            onSelect(selectedIndex + 1);
        }
        setMainShortcutCounter(count => count + 1);
    });

    const keyUp = (event: KeyboardEvent<HTMLDivElement>) => {
        if (mainShortcutOn) {
            if (!matchShortcutModifiers(settings.allShortcuts.openCcv, event)) {
                if (mainShortcutCounter > 1) {
                    onActivate(selectedIndex);
                }
                setMainShortcutOn(false);
            }
        }
    };

    const keyDown = (event: KeyboardEvent<HTMLDivElement>) => {
        if (event.key === 'Enter') {
            onActivate(selectedIndex);
            return;
        }
        if (event.key === 'Escape') {
            onHide();
            return;
        }
        if (event.key === 'ArrowUp') {
            if (mainShortcutOn) {
                setMainShortcutCounter(count => count + 1);
            }
            onSelect(selectedIndex - 1)
            return;
        }
        if (event.key === 'ArrowDown') {
            if (mainShortcutOn) {
                setMainShortcutCounter(count => count + 1);
            }
            onSelect(selectedIndex + 1);
            return;
        }
        if (event.ctrlKey && event.key >= '1' && event.key <= '9') {
            onActivate(parseInt(event.key) - 1);
            return;
        }
    };
    
    return (
        <div 
            className={styles.container} 
            onKeyDown={keyDown}
            onKeyUp={keyUp}
            ref={containerRef} 
            tabIndex={0}>
           { children }
        </div>
    );
}
