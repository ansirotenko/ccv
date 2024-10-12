import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { ComponentProps, useContext, useEffect, useState } from 'react';
import { check, Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { AboutContext } from '../../common/AboutContext';
import { Message } from 'primereact/message';
import { ProgressBar } from 'primereact/progressbar';

import styles from './UpdateDialog.module.css';

interface UpdateDialogProps extends ComponentProps<'div'> {
    notifyUpToDate: boolean 
}

export function UpdateDialog({ notifyUpToDate }: UpdateDialogProps) {
    const [loading, setLoading] = useState<boolean>(true);
    const [update, setUpdate] = useState<Update | null>(null);
    const [confirmed, setConfirmed] = useState(false);
    const [updateIsRunning, setUpdateIsRunning] = useState(false);
    const [updateStatus, setUpdateStatus] = useState<string>("PENDING");
    const [updateError, setUpdateError] = useState<string | undefined>(undefined);
    const [contentLength, setContentLength] = useState<number | undefined>();
    const [downloaded, setDownloaded] = useState<number | undefined>();
    const aboutData = useContext(AboutContext);

    useEffect(() => {
        check().then(update => {
            setUpdate(update);
        })
        .finally(() => {
            setLoading(false);
        })
    }, [])

    // TODO test functionality

    if (loading) {
        return <></>
    }

    function getValue() {
        if (!contentLength || contentLength < 0) {
            return 0;
        }

        return (downloaded || 0) / contentLength;
    }

    function updatingContent() {
        if (updateError) {
            return <Message severity="error" className={styles.error} text={updateError} />;
        }
        return (
            <>
                <p>
                    {updateStatus}
                </p>
                <ProgressBar value={getValue()} className={styles.progress}></ProgressBar>
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

    async function installUpdate() {
        setConfirmed(true)
        setUpdateIsRunning(true);

        try {
            await update?.downloadAndInstall((event) => {
                switch (event.event) {
                    case 'Started':
                        setUpdateStatus("STARTED");
                        setContentLength(event.data.contentLength);
                        break;
                    case 'Progress':
                        setUpdateStatus("DOWNLOADING");
                        setDownloaded(d => (d || 0) + event.data.chunkLength);
                        break;
                    case 'Finished':
                        setUpdateStatus("INSTALLING");
                        break;
                  }
            });
            await relaunch();
        } catch (e: any) {
            // TODO see what happens
            setUpdateError(e.toString());
        }
    }

    if (update != null && !confirmed) {
        return (
            <Dialog
                header={<>Version <b>{update?.version}</b> is available, you have <b>{aboutData?.version}</b></>}
                visible={true}
                style={{ width: '320px' }}
                onHide={() => {}}
                footer={ 
                    <>
                        <Button className='p-button-info p-button-outlined' onClick={() => setConfirmed(true)}>Cacnel</Button>
                        <Button onClick={installUpdate}>Install</Button>
                    </>
                }
                draggable={false}
                resizable={false}
                closable={false}
            >
                <div>Release notes:</div>
                <p className={styles.manifestBody}>
                    {update?.body}
                </p>
                <div>Would you like to update Ccv?</div>
            </Dialog>
        );
    }
    
    if (update == null && notifyUpToDate && !confirmed) {
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
