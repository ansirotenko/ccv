import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { ComponentProps, useContext } from 'react';
import { SettingsContext } from '../../common/SettingsContext';
import { shortcutDisplay } from '../../common/keyboard';
import { AboutContext } from '../../common/AboutContext';

interface OccupiedShortcutDialogProps extends ComponentProps<'div'> {
    onOk: () => void;
}

export function OccupiedShortcutDialog({ onOk }: OccupiedShortcutDialogProps) {
    const settings = useContext(SettingsContext);
    const about = useContext(AboutContext);

    return (
        <Dialog
            header="Ccv default shortcut is occupied"
            visible={true}
            style={{ width: '320px' }}
            onHide={() => {}}
            footer={<Button onClick={onOk}>Got it</Button>}
            draggable={false}
            resizable={false}
            closable={false}
        >
            <p>
                The default shortcut for open application {' '}
                <b><em>{shortcutDisplay(settings.allShortcuts.openCcv, about!.os)}</em></b>
                {' '}is already occupied in your system. Please change it in the settings (button at the top right corner).
            </p>
        </Dialog>
    );
}
