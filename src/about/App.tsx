import { useContext, useState } from 'react';
import { Button } from 'primereact/button';
import { Skeleton } from 'primereact/skeleton';
import { BlockUI } from 'primereact/blockui';
import { useSubscribeEvent, HIGHLIGHT_REPORT_BUG } from '../common/events';
import { hideAboutWindow, showPrimaryWindow, openAnything } from '../common/commands';
import { AboutContext } from '../common/AboutContext';

import styles from './App.module.css';

function App() {
    const [reportBugHightlighted, setReportBugHightlighted] = useState<boolean>(false);
    const aboutData = useContext(AboutContext);
    useSubscribeEvent<string>(HIGHLIGHT_REPORT_BUG, () => setReportBugHightlighted(true));

    async function closeAbout() {
        await hideAboutWindow();
        await showPrimaryWindow();
    }

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
                <Button className={`pi pi-times ${styles.toolbarButton}`} onClick={closeAbout} />
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
                                    openAnything(aboutData.homepage);
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
                                    openAnything(aboutData.appDirectory!);
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
                                    openAnything(aboutData.appDataDirectory!);
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
                                    openAnything(aboutData.appLogsDirectory!);
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
                            openAnything('https://github.com/ansirotenko/ccv/issues/new');
                        }}
                    >
                        github
                    </a>{' '}
                    or via{' '}
                    <a
                        href="#"
                        onClick={() => {
                            openAnything('mailto:panshirotenski@gmail.com');
                        }}
                    >
                        email
                    </a>
                    . It would be very helpfull if you can attach logs.
                </div>
                <BlockUI blocked={reportBugHightlighted}>
                    <section className={styles.okButtonContainer}>
                        <Button className={styles.okButton} onClick={closeAbout}>
                            Ok
                        </Button>
                    </section>
                </BlockUI>
            </div>
        </div>
    );
}

export default App;
