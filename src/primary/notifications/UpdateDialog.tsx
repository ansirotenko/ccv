import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { ComponentProps, useContext, useEffect, useState } from 'react';
import {
    checkUpdate,
    installUpdate,
    onUpdaterEvent,
    UpdateManifest
  } from '@tauri-apps/api/updater';
import { relaunch } from '@tauri-apps/api/process';
import { AboutContext } from '../../common/AboutContext';
import { Message } from 'primereact/message';
import { ProgressBar } from 'primereact/progressbar';

import styles from './UpdateDialog.module.css';

interface UpdateDialogProps extends ComponentProps<'div'> {
    notifyIfUpToDate: boolean 
}

export function UpdateDialog({ notifyIfUpToDate }: UpdateDialogProps) {
    const [shouldUpdate, setShouldUpdate] = useState<boolean | undefined>();
    const [upateManifest, setUpateManifest] = useState<UpdateManifest | undefined>();
    const [confirmed, setConfirmed] = useState(false);
    const [updateIsRunning, setUpdateIsRunning] = useState(false);
    const [updateStatus, setUpdateStatus] = useState<string>("PENDING");
    const [updateError, setUpdateError] = useState<string | undefined>(undefined);
    const aboutData = useContext(AboutContext);

    useEffect(() => {
        async function listen() {
            checkUpdate().then(({ shouldUpdate, manifest }) => {
                setShouldUpdate(shouldUpdate);
                setUpateManifest(manifest);
            })
    
            const unlisten = await onUpdaterEvent(({ error, status }) => {
                setUpdateError(error);
                setUpdateStatus(status);
            })

            return unlisten;
        }
        
        const promise = listen();
        return () => {
            promise.then((c) => c());
        };
    }, [])

    function updatingContent() {
        if (updateError) {
            return <Message severity="error" className={styles.error} text={updateError} />;
        }
        return (
            <>
                <p>
                    {updateStatus}
                </p>
                <ProgressBar mode="indeterminate" className={styles.progress}></ProgressBar>
            </>
        )
    }

    if (updateIsRunning) {
        return (
            <Dialog
                header={<>Installing update</>}
                visible={true}
                style={{ width: '320px' }}
                onHide={() => {}}
                draggable={false}
                resizable={false}
                closable={false}
                footer={updateError && <Button onClick={() => setUpdateIsRunning(false)}>Close</Button>}
            >
                {updatingContent()}
            </Dialog>
        );
    }

    if (shouldUpdate && !confirmed) {
        return (
            <Dialog
                header={<>Version <b>{upateManifest?.version}</b> is available, you have <b>{aboutData?.version}</b></>}
                visible={true}
                style={{ width: '320px' }}
                onHide={() => {}}
                footer={ 
                    <>
                        <Button className='p-button-info p-button-outlined' onClick={() => setConfirmed(true)}>Cacnel</Button>
                        <Button onClick={async () => {
                            setConfirmed(true)
                            setUpdateIsRunning(true);
                            await installUpdate();
                            await relaunch();
                        }}>Install</Button>
                    </>
                }
                draggable={false}
                resizable={false}
                closable={false}
            >
                <div>Release notes:</div>
                <p className={styles.manifestBody}>
                    {upateManifest?.body}
                </p>
                <div>Would you like to update Ccv?</div>
            </Dialog>
        );
    }
    
    if (shouldUpdate == false && notifyIfUpToDate && !confirmed) {
        return (
            <Dialog
                header="Ccv is up to date"
                visible={true}
                style={{ width: '320px' }}
                onHide={() => {}}
                footer={<Button onClick={() => setConfirmed(true)}>Close</Button>}
                draggable={false}
                resizable={false}
                closable={false}
            >
                <p>
                    Your current version is <b>{aboutData?.version}</b>
                </p>
            </Dialog>
        );
    }

    return <></>
}
