import { useContext } from 'react';
import { SettingsContext } from '../../common/SettingsContext';
import { setSettings } from '../../common/commands';
import { Settings } from '../../common/contract';
import { WelcomeDialog } from './WelcomeDialog';
import { UpdateDialog } from './UpdateDialog';
import { OccupiedShortcutDialog } from './OccupiedShortcutDialog';

export async function eraseNotification(name: string, settings: Settings) {
    const newNotifications = settings.notifications?.filter((x) => x != name);
    await setSettings({ ...settings, notifications: newNotifications });
}

const WELCOME = 'welcome';
const OCCUPIED_SHORTCUT_NOTIFICATION = 'occupiedShortcut';

export function Notifications() {
    const settings = useContext(SettingsContext);

    if (settings && settings.notifications?.length) {
        const name = settings.notifications[0];

        switch (name) {
            case WELCOME:
                return <WelcomeDialog onOk={() => eraseNotification(WELCOME, settings)}></WelcomeDialog>;
            case OCCUPIED_SHORTCUT_NOTIFICATION:
                return (
                    <OccupiedShortcutDialog
                        onOk={() => eraseNotification(OCCUPIED_SHORTCUT_NOTIFICATION, settings)}
                    ></OccupiedShortcutDialog>
                );
        }
    }

    return <UpdateDialog notifyUpToDate={false}></UpdateDialog>;
}
