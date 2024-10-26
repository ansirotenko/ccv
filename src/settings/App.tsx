import { Button } from 'primereact/button';
import { Fieldset } from 'primereact/fieldset';
import { useRef, useState, KeyboardEvent, useContext, ReactNode } from 'react';
import { InputText } from 'primereact/inputtext';
import { Calendar } from 'primereact/calendar';
import { RadioButton } from 'primereact/radiobutton';
import { Toast } from 'primereact/toast';
import { ConfirmDialog, confirmDialog } from 'primereact/confirmdialog';
import * as log from '@tauri-apps/plugin-log';
import { AppError, DeleteSummary, Settings, Shortcut } from '../common/contract';
import { SettingsContext } from '../common/SettingsContext';
import { shortcutDisplay, shortcutFromEvent } from '../common/keyboard';
import { hideSettingsWindow, showPrimaryWindow, removeCopyItems, removeCopyItemsOlder, setSettings } from '../common/commands';
import { Checkbox } from 'primereact/checkbox';
import { AboutContext } from '../common/AboutContext';
import itemIdImage from '../assets/itemId.png';

import styles from './App.module.css';
import { Tooltip } from 'primereact/tooltip';

function App() {
    const settings = useContext(SettingsContext);
    const [deleteDate, setDeleteDate] = useState<Date | null>(null);
    const [selectedIds, setSelectedIds] = useState<string>('');
    const shortcutDialog = useRef<HTMLDivElement>(null);
    const shortcutValue = useRef<HTMLInputElement>(null);
    const toast = useRef<Toast>(null);
    const newShortcutRef = useRef<Shortcut | null>();
    const about = useContext(AboutContext);

    async function closeSettings() {
        await showPrimaryWindow();
        await hideSettingsWindow();
    }

    const saveSettings = async (newSettings: Settings) => {
        try {
            await setSettings(newSettings);
            showSuccess(`Settings were changed`, 500);
        } catch (e) {
            const appError = e as AppError;
            log.error(appError.message);
            showError(`Failed to save settings: ${appError.message}`);
        }
    };

    const showSuccess = (message: string | ReactNode, life: number) => {
        if (toast.current) {
            toast.current.show({ severity: 'success', summary: 'Success', detail: message, life: life });
        }
    };

    const showError = (message: string) => {
        if (toast.current) {
            toast.current.show({ severity: 'error', summary: 'Failure', detail: message, life: 4000 });
        }
    };

    const keyDown = (event: KeyboardEvent<HTMLDivElement>) => {
        const newShortcut = shortcutFromEvent(event);
        onShortcutChanged(newShortcut);
        event.preventDefault();
    };

    const confirmShortcuts = () => {
        confirmDialog({
            accept: () => {
                saveSettings({ ...settings!, allShortcuts: { ...settings?.allShortcuts, openCcv: newShortcutRef.current! } });
            },
            acceptLabel: 'Apply',
            acceptClassName: 'acceptButton',
            rejectClassName: 'p-button-info p-button-outlined',
            rejectLabel: 'Cancel',
            header: 'Press new shortcut',
            message: (
                <div className={styles.shortcutEdit} onKeyDown={keyDown} tabIndex={0} ref={shortcutDialog}>
                    <label>Old combination:</label> <span>{shortcutDisplay(settings?.allShortcuts.openCcv, about!.os)}</span>
                    <label>New combination:</label> <InputText ref={shortcutValue} disabled={true} />
                </div>
            ),
            onShow: () => {
                shortcutDialog.current?.focus();
                onShortcutChanged(null);
            },
            onClick: () => {
                shortcutDialog.current?.focus();
            },
        });
    };

    const onShortcutChanged = (newShortcut: Shortcut | null) => {
        newShortcutRef.current = newShortcut;
        if (!newShortcut || !newShortcut.code) {
            (document.getElementsByClassName('acceptButton')[0] as HTMLButtonElement).disabled = true;
        } else {
            (document.getElementsByClassName('acceptButton')[0] as HTMLButtonElement).disabled = false;
        }

        if (shortcutValue.current) {
            shortcutValue.current.value = !newShortcut ? '' : shortcutDisplay(newShortcut, about!.os);
        }
    };

    const confirmDeleteIds = () => {
        confirmDialog({
            accept: async () => {
                try {
                    if (selectedIds) {
                        const deleteSummary = await removeCopyItems(selectedIds);
                        showSuccessfulDeletion(deleteSummary);
                    } else {
                        const errorMessage = `Ids are empty`;
                        log.error(errorMessage);
                        showError(errorMessage);
                    }
                } catch (e) {
                    const appError = e as AppError;
                    log.error(appError.message);
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
                        const deleteSummary = await removeCopyItemsOlder(deleteDate);
                        showSuccessfulDeletion(deleteSummary);
                    } else {
                        const errorMessage = `Deletion date is null`;
                        log.error(errorMessage);
                        showError(errorMessage);
                    }
                } catch (e) {
                    const appError = e as AppError;
                    log.error(appError.message);
                    showError(`Deleton failed: ${appError.message}`);
                }
            },
            icon: 'pi pi-exclamation-triangle',
            header: 'Deletion',
            message: <>Are you sure you want to clear history up to {deleteDate?.toLocaleString()}?</>,
        });
    };

    function showSuccessfulDeletion(deleteSummary: DeleteSummary) {
        const noteCurrentValue = deleteSummary.isActiveRestored ? ' Current copy buffer value was restored as first item. ' : '';
        const plural = deleteSummary.deletedCount !== 1 ? 's' : '';
        showSuccess(
            `${deleteSummary.deletedCount} item${plural} were deleted.${noteCurrentValue}`,
            deleteSummary.isActiveRestored ? 6000 : 3000,
        );
    }

    return (
        <div className={styles.container}>
            <Toast ref={toast} position="center" />
            <ConfirmDialog style={{ width: '400px' }} />
            <div className={styles.toolbar}>
                <span className={styles.toolbarTitle} data-tauri-drag-region>
                    Settings
                </span>
                <Button className={`pi pi-times ${styles.toolbarButton}`} onClick={closeSettings} />
            </div>
            <div className={styles.content}>
                <Fieldset legend="General">
                    <div className={styles.generalSettings}>
                        <label>Theme</label>
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
                        <label>Autostart</label>
                        <Checkbox
                            checked={settings?.autostart}
                            onChange={(e) => {
                                if (e.checked != settings?.autostart) {
                                    saveSettings({ ...settings!, autostart: !!e.checked });
                                }
                            }}
                        />
                    </div>
                </Fieldset>
                <Fieldset legend="Shortcuts">
                    <div className="p-inputgroup">
                        <span className="p-inputgroup-addon">Open ccv</span>
                        <InputText value={shortcutDisplay(settings?.allShortcuts.openCcv, about!.os)} disabled={true} />
                        <Button className={`${styles.settingsButton} changeOpenCcvShortcut`} onClick={confirmShortcuts}>
                            Change
                        </Button>
                        <Tooltip
                            target=".changeOpenCcvShortcut"
                            position="left"
                            style={{ fontSize: 13, maxWidth: '380px', textAlign: 'center' }}
                            showOnDisabled={true}
                        >
                            Change shortcut to open ccv.
                        </Tooltip>
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
                            hideOnDateTimeSelect={true}
                        />
                        <Button
                            className={`${styles.settingsButton} deleteByTime`}
                            disabled={deleteDate == null}
                            onClick={confirmDeleteOlder}
                        >
                            Delete older
                        </Button>
                        <Tooltip
                            target=".deleteByTime"
                            position="left"
                            style={{ fontSize: 13, maxWidth: '380px', textAlign: 'center' }}
                            showOnDisabled={true}
                        >
                            {`Clear history up to selected date. ${deleteDate == null ? 'Please specify date.' : ''}`}
                        </Tooltip>
                    </div>
                    <div className="p-inputgroup">
                        <InputText placeholder="12345..." value={selectedIds} onChange={(e) => setSelectedIds(e.target.value)} />
                        <Button className={`${styles.settingsButton} deleteById`} disabled={!selectedIds} onClick={confirmDeleteIds}>
                            Delete ids
                        </Button>
                        <Tooltip
                            target=".deleteById"
                            position="left"
                            style={{ fontSize: 13, maxWidth: '380px', textAlign: 'center' }}
                            showOnDisabled={true}
                        >
                            Delete items by ids separated by comma. Id of the item could be found at the bottom of primary view.
                            <img src={itemIdImage} width="350px" />
                        </Tooltip>
                    </div>
                </Fieldset>
                <section className={styles.okButtonContainer}>
                    <Button className={styles.okButton} onClick={closeSettings}>
                        Ok
                    </Button>
                </section>
            </div>
        </div>
    );
}

export default App;
