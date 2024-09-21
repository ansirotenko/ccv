import { useBackend } from './useBackend';
import { Button } from 'primereact/button';
import { Fieldset } from 'primereact/fieldset';
import { useRef, useState, KeyboardEvent, useContext } from 'react';
import { InputText } from 'primereact/inputtext';
import { Calendar } from 'primereact/calendar';
import { RadioButton } from 'primereact/radiobutton';
import { Toast } from 'primereact/toast';
import { ConfirmDialog, confirmDialog } from 'primereact/confirmdialog';
import { error as logError } from 'tauri-plugin-log-api';
import { AppError, Settings, Shortcut } from '../api';
import SettingsContext from '../common/SettingsContext';

import styles from './App.module.css';

function App() {
    const settings = useContext(SettingsContext);
    const [deleteDate, setDeleteDate] = useState<Date | null>(null);
    const [selectedIds, setSelectedIds] = useState<string>('');
    const keybindingDialog = useRef<HTMLDivElement>(null);
    const keyBindingValue = useRef<HTMLSpanElement>(null);
    const toast = useRef<Toast>(null);
    const newShortcutRef = useRef<Shortcut | null>();
    const backend = useBackend();

    const saveSettings = async (newSettings: Settings) => {
        try {
            await backend.setSettings(newSettings);
            showSuccess(`Settings were changed`);
        } catch (e) {
            const appError = e as AppError;
            logError(appError.message);
            showError(`Failed to save settings: ${appError.message}`);
        }
    };

    const showSuccess = (message: string) => {
        if (toast.current) {
            toast.current.show({ severity: 'success', summary: 'Success', detail: message, life: 2000 });
        }
    };

    const showError = (message: string) => {
        if (toast.current) {
            toast.current.show({ severity: 'error', summary: 'Failure', detail: message, life: 2000 });
        }
    };

    const keyDown = (event: KeyboardEvent<HTMLDivElement>) => {
        const newShortcut: Shortcut = {
            altKey: event.altKey,
            ctrlKey: event.ctrlKey,
            shiftKey: event.shiftKey,
            metaKey: event.metaKey,
            code: !event.key || event.key == 'Control' || event.key == 'Alt' || event.key == 'Shift'
                ? undefined
                : event.code 
        }
        onShortcutChanged(newShortcut);
        event.preventDefault();
    };

    const confirmKeybindings = () => {
        confirmDialog({
            accept: () => {
                saveSettings({ ...settings!, keybindings: { ...settings?.keybindings, openCcv: newShortcutRef.current! } });
            },
            acceptLabel: 'Apply',
            acceptClassName: 'acceptButton',
            rejectLabel: 'Cancel',
            header: 'Press new keybinding',
            message: (
                <div className={styles.keybindingEdit} onKeyDown={keyDown} tabIndex={0} ref={keybindingDialog}>
                    <label>Old combination:</label> <span>{backend.getShortcutDisplay(settings?.keybindings.openCcv)}</span>
                    <label>New combination:</label> <span ref={keyBindingValue}></span>
                </div>
            ),
            onShow: () => {
                keybindingDialog.current?.focus();
                onShortcutChanged(null);
            },
            onClick: () => {
                keybindingDialog.current?.focus();
            },
        });
    };

    const onShortcutChanged = (newShortcut: Shortcut | null) => {
        newShortcutRef.current = newShortcut;
        if (!newShortcut) {
            if (keyBindingValue.current) {
                keyBindingValue.current.innerHTML = "";
            }
            (document.getElementsByClassName('acceptButton')[0] as HTMLButtonElement).disabled = true;
        } else {
            if (keyBindingValue.current) {
                keyBindingValue.current.innerHTML = backend.getShortcutDisplay(newShortcut);
            }
            (document.getElementsByClassName('acceptButton')[0] as HTMLButtonElement).disabled = false;
        }
    };

    const confirmDeleteIds = () => {
        confirmDialog({
            accept: async () => {
                try {
                    if (selectedIds) {
                        await backend.removeCopyItems(selectedIds);
                        showSuccess(`Deletion completed`);
                    } else {
                        const errorMessage = `Ids are empty`;
                        logError(errorMessage);
                        showError(errorMessage);
                    }
                } catch (e) {
                    const appError = e as AppError;
                    logError(appError.message);
                    showError(`Deleton failed: ${appError.message}`);
                }
            },
            icon: 'pi pi-exclamation-triangle',
            header: 'Deletion',
            message: <>Are you sure you want to remove ids {selectedIds}?</>,
        });
    };

    const confirmDeleteOlder = () => {
        confirmDialog({
            accept: async () => {
                try {
                    if (deleteDate) {
                        await backend.removeCopyItemsOlder(deleteDate);
                        showSuccess(`Deletion completed`);
                    } else {
                        const errorMessage = `Deletion date is null`;
                        logError(errorMessage);
                        showError(errorMessage);
                    }
                } catch (e) {
                    const appError = e as AppError;
                    logError(appError.message);
                    showError(`Deleton failed: ${appError.message}`);
                }
            },
            icon: 'pi pi-exclamation-triangle',
            header: 'Deletion',
            message: <>Are you sure you want to clear history up to {deleteDate?.toLocaleString()}?</>,
        });
    };

    return (
        <div className={styles.container}>
            <Toast ref={toast} position="center" />
            <ConfirmDialog style={{ width: '400px' }} />
            <div className={styles.toolbar}>
                <span className={styles.toolbarTitle} data-tauri-drag-region>
                    Settings
                </span>
                <Button className={`pi pi-times ${styles.toolbarButton}`} onClick={backend.hideSettingsWindow} />
            </div>
            <div className={styles.content}>
                <Fieldset legend="Theme">
                    <div className={styles.themeContainer}>
                        <div className={styles.theme}>
                            <RadioButton
                                inputId="lightTheme"
                                name="light"
                                onChange={() => {
                                    if (settings && settings.theme !== 'Light') {
                                        saveSettings({ ...settings!, theme: 'Light' });
                                    }
                                }}
                                checked={settings?.theme === 'Light'}
                            />
                            <label htmlFor="lightTheme">Light</label>
                        </div>
                        <div className={styles.theme}>
                            <RadioButton
                                inputId="darkTheme"
                                onChange={() => {
                                    if (settings && settings.theme !== 'Dark') {
                                        saveSettings({ ...settings!, theme: 'Dark' });
                                    }
                                }}
                                checked={settings?.theme === 'Dark'}
                            />
                            <label htmlFor="darkTheme">Dark</label>
                        </div>
                    </div>
                </Fieldset>
                <Fieldset legend="Key bindings">
                    <div className="p-inputgroup">
                        <span className="p-inputgroup-addon">Open ccv</span>
                        <InputText value={backend.getShortcutDisplay(settings?.keybindings.openCcv)} disabled={true} />
                        <Button
                            className={styles.settingsButton}
                            onClick={confirmKeybindings}
                            tooltip="Change keybinding to open ccv"
                            tooltipOptions={{ position: 'left', style: { fontSize: 13 } }}
                        >
                            Change
                        </Button>
                    </div>
                </Fieldset>
                <Fieldset legend="Deletion">
                    <div className="p-inputgroup">
                        <Calendar
                            value={deleteDate}
                            onChange={(e) => {
                                if (e.value) {
                                    const date = new Date(e.value);
                                    date.setSeconds(0);
                                    setDeleteDate(date);
                                } else {
                                    setDeleteDate(null);
                                }
                            }}
                            showButtonBar
                            showTime
                            hourFormat="24"
                        />
                        <Button
                            className={styles.settingsButton}
                            disabled={deleteDate == null}
                            onClick={confirmDeleteOlder}
                            tooltip={`Clear history up to selected date${deleteDate == null ? '. Please specify date' : ''}`}
                            tooltipOptions={{ position: 'left', style: { fontSize: 13 }, showOnDisabled: true }}
                        >
                            Delete older
                        </Button>
                    </div>
                    <div className="p-inputgroup">
                        <InputText placeholder="55,555,5,5555..." value={selectedIds} onChange={(e) => setSelectedIds(e.target.value)} />
                        <Button
                            className={styles.settingsButton}
                            disabled={!selectedIds}
                            onClick={confirmDeleteIds}
                            tooltip={`Delete items by ids separated by comma${!selectedIds ? '. Please specify ids' : ''}`}
                            tooltipOptions={{ position: 'left', style: { fontSize: 13 }, showOnDisabled: true }}
                        >
                            Delete ids
                        </Button>
                    </div>
                </Fieldset>
            </div>
        </div>
    );
}

export default App;
