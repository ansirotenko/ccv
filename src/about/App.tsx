import { useEffect, useState } from 'react';
import { AboutData } from '../api';
import { useBackend } from './useBackend';
import { Button } from 'primereact/button';
import { Skeleton } from 'primereact/skeleton';
import { BlockUI } from 'primereact/blockui';
import { HIGHLIGHT_REPORT_BUG } from '../events';
import { useSubscribeEvent } from '../common/useSubscribeEvent';

import styles from './App.module.css';

function App() {
    const [reportBugHightlighted, setReportBugHightlighted] = useState<boolean>(false);
    const [aboutData, setSetAboutData] = useState<AboutData | undefined>();
    const backend = useBackend();
    useSubscribeEvent<string>(HIGHLIGHT_REPORT_BUG, () => setReportBugHightlighted(true));

    useEffect(() => {
        backend.getAboutData().then((data) => {
            setSetAboutData(data);
        });
    }, []);

    return (
        <div
            className={styles.container}
            onClick={() => {
                setReportBugHightlighted(false);
            }}
        >
            <div className={styles.toolbar}>
                <span className={styles.toolbarTitle} data-tauri-drag-region>
                    About
                </span>
                <Button className={`pi pi-times ${styles.toolbarButton}`} onClick={backend.hideAboutWindow} />
            </div>
            <div className={styles.content}>
                <BlockUI blocked={reportBugHightlighted}>
                    <section>{aboutData ? aboutData.text : <Skeleton width="100%" height="50px" />}</section>
                    <br />
                    <section>
                        <strong>Version:</strong>
                        {aboutData ? aboutData.version : <Skeleton width="40px" height="16px" />}
                    </section>
                    <section>
                        <strong>Authors:</strong>
                        {aboutData ? aboutData.authors : <Skeleton width="70px" height="16px" />}
                    </section>
                    <section>
                        <strong>Homepage:</strong>
                        {aboutData ? (
                            <a
                                href="#"
                                onClick={() => {
                                    backend.open(aboutData.homepage);
                                }}
                            >
                                {aboutData.homepage}
                            </a>
                        ) : (
                            <Skeleton width="80px" height="16px" />
                        )}
                    </section>
                    <section>
                        <strong>App directory:</strong>
                        {aboutData?.appDirectory ? (
                            <em
                                onClick={() => {
                                    backend.open(aboutData.appDirectory!);
                                }}
                            >
                                {aboutData.appDirectory}
                            </em>
                        ) : (
                            <Skeleton width="80px" height="16px" />
                        )}
                    </section>
                    <section>
                        <strong>App data directory:</strong>
                        {aboutData?.appDataDirectory ? (
                            <em
                                onClick={() => {
                                    backend.open(aboutData.appDataDirectory!);
                                }}
                            >
                                {aboutData.appDataDirectory}
                            </em>
                        ) : (
                            <Skeleton width="80px" height="16px" />
                        )}
                    </section>
                    <section>
                        <strong>App logs directory:</strong>
                        {aboutData?.appLogsDirectory ? (
                            <em
                                onClick={() => {
                                    backend.open(aboutData.appLogsDirectory!);
                                }}
                            >
                                {aboutData.appLogsDirectory}
                            </em>
                        ) : (
                            <Skeleton width="80px" height="16px" />
                        )}
                    </section>
                </BlockUI>
                <div className={`${styles.reportBug}`}>
                    Feel free to report any issue or bug found on{' '}
                    <a
                        href="#"
                        onClick={() => {
                            backend.open('https://github.com/ansirotenko/ccv/issues/new');
                        }}
                    >
                        github
                    </a>{' '}
                    or via{' '}
                    <a
                        href="#"
                        onClick={() => {
                            backend.open('mailto:panshirotenski@gmail.com');
                        }}
                    >
                        email
                    </a>
                    . It would be very helpfull if you can attach logs.
                </div>
                <BlockUI blocked={reportBugHightlighted}>
                    <section className={styles.contentButtonContainer}>
                        <Button className={styles.contentButton} onClick={backend.hideAboutWindow}>
                            Ok
                        </Button>
                    </section>
                </BlockUI>
            </div>
        </div>
    );
}

export default App;
