import { ComponentProps, useState, useContext, useEffect, useRef } from 'react';
import { MainShortcutPressedPayload } from '../../common/contract';
import { useSubscribeEvent, MAIN_SHORTCUT_PRESSED_EVENT } from '../../common/events';
import { hasModifers, matchShortcutModifiers } from '../../common/keyboard';
import { SettingsContext } from '../../common/SettingsContext';

import styles from './Container.module.css';

interface ContainerProps extends Omit<ComponentProps<'div'>, 'onSelect'> {
    selectedIndex: number;
    onSelect: (index: number) => void;
    onActivate: (index: number) => void;
    onHide: () => void;
}

type EventsHadler = {
    keyDown: (event: KeyboardEvent) => void,
    keyUp: (event: KeyboardEvent) => void,
}

export function Container({ selectedIndex, onSelect, onActivate, onHide, children }: ContainerProps) {
    const [mainShortcutOn, setMainShortcutOn] = useState<boolean>(false);
    const [mainShortcutCounter, setMainShortcutCounter] = useState<number>(0);
    const settings = useContext(SettingsContext);
    const eventsHandler = useRef<EventsHadler>(undefined);

    useSubscribeEvent<MainShortcutPressedPayload>(MAIN_SHORTCUT_PRESSED_EVENT, (mainShortcutPressedPayload) => {
        if (mainShortcutPressedPayload.changedFromHiddenToVisile) {
            if (hasModifers(settings.allShortcuts.openCcv)) {
                setMainShortcutOn(true);
                setMainShortcutCounter(0);
            }
        } else {
            onSelect(selectedIndex + 1);
        }
        setMainShortcutCounter((count) => count + 1);
    });

    useEffect(() => {
        eventsHandler.current = {
            keyUp: (event: KeyboardEvent) => {
                if (mainShortcutOn) {
                    if (!matchShortcutModifiers(settings.allShortcuts.openCcv, event)) {
                        if (mainShortcutCounter > 1) {
                            onActivate(selectedIndex);
                        }
                        setMainShortcutOn(false);
                    }
                }
            },
            keyDown: (event: KeyboardEvent) => {
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
                        setMainShortcutCounter((count) => count + 1);
                    }
                    onSelect(selectedIndex - 1);
                    return;
                }
                if (event.key === 'ArrowDown') {
                    if (mainShortcutOn) {
                        setMainShortcutCounter((count) => count + 1);
                    }
                    onSelect(selectedIndex + 1);
                    return;
                }
                if (event.ctrlKey && event.key >= '1' && event.key <= '9') {
                    onActivate(parseInt(event.key) - 1);
                    return;
                }
            }
        }
    }, [selectedIndex, onSelect, onActivate, onHide])

    const keyUp = (event: KeyboardEvent) => {
        if (eventsHandler.current) {
            eventsHandler.current.keyUp(event);
        }
    };

    const keyDown = (event: KeyboardEvent) => {
        if (eventsHandler.current) {
            eventsHandler.current.keyDown(event);
        }
    };

    useEffect(() => {
        addEventListener("keydown", keyDown);
        addEventListener("keyup", keyUp);

        return () => {
            removeEventListener("keydown", keyDown);
            removeEventListener("keyup", keyUp);
        }
    }, [])

    return (
        <div className={styles.container}>
            {children}
        </div>
    );
}
