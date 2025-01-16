import { ComponentProps, useContext, useEffect, useRef, useState } from 'react';
import { AboutData, Settings, Theme } from '../../common/contract';
import { PrimeReactContext } from 'primereact/api';
import { SettingsContext } from '../SettingsContext';
import { AboutContext } from '../AboutContext';
import { invoke } from '@tauri-apps/api/core';
import { ProgressSpinner } from 'primereact/progressspinner';
import { Message } from 'primereact/message';
import { useSubscribeEvent, SETTINGS_UPDATED } from '../events';

import 'primeicons/primeicons.css';
import styles from './AppWrapper.module.css';
import { getAboutData } from '../commands';

async function getSettings() {
    return await invoke<Settings>('get_settings');
}

let attempts = 20;

function AppWrapper({ children }: ComponentProps<'div'>) {
    const [settings, setSettings] = useState<Settings | undefined>();
    const [aboutData, setAboutData] = useState<AboutData | undefined>();
    const [hasError, setHasError] = useState<boolean>();
    const [oldTheme, setOldTheme] = useState<Theme>('Light');
    const timeout = useRef<ReturnType<typeof setTimeout>>(undefined);
    const { changeTheme } = useContext(PrimeReactContext);
    useSubscribeEvent<Settings>(SETTINGS_UPDATED, (newSettings) => setSettings(newSettings));

    useEffect(() => {
        getAboutData().then((aboutData) => {
            setAboutData(aboutData);
        });
        loadSettingsAttempt();
    }, []);

    useEffect(() => {
        if (settings) {
            if (oldTheme !== settings.theme) {
                changeTheme!(oldTheme, settings.theme, 'theme-link', () => {});
                setOldTheme(settings.theme);
            }
        }
    }, [settings]);

    function loadSettingsAttempt() {
        timeout.current = setTimeout(async () => {
            const newSettings = await getSettings();
            if (newSettings) {
                if (timeout.current) {
                    clearTimeout(timeout.current);
                }
                setSettings(newSettings);
            } else {
                attempts--;
                if (attempts < 0) {
                    setHasError(true);
                } else {
                    loadSettingsAttempt();
                }
            }
        }, 100);
    }

    if (hasError) {
        return (
            <div className={styles.failedContainer}>
                <Message severity="error" text="Failed to load application. Please check logs." />
            </div>
        );
    }

    if (!settings) {
        return (
            <div className={styles.loadingContainer}>
                <ProgressSpinner />
            </div>
        );
    }

    return (
        <SettingsContext.Provider value={settings}>
            <AboutContext.Provider value={aboutData}>{children}</AboutContext.Provider>
        </SettingsContext.Provider>
    );
}

export default AppWrapper;
