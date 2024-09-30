import { Button } from 'primereact/button';
import { Dialog } from 'primereact/dialog';
import { ComponentProps, useContext } from 'react';
import { SettingsContext } from '../../common/SettingsContext';
import { shortcutDisplay } from '../../common/keyboard';

interface WelcomeDialogProps extends ComponentProps<'div'> {
    onOk: () => void;
}

export function WelcomeDialog({ onOk }: WelcomeDialogProps) {
    const settings = useContext(SettingsContext);

    return (
        <Dialog
            header="Welcome to CCV"
            visible={true}
            style={{ width: '320px' }}
            onHide={() => {}}
            footer={<Button onClick={onOk}>Got it</Button>}
            draggable={false}
            resizable={false}
            closable={false}
        >
            <p>
                Ccv will save the history of your clipboard exchange. Find desired item using search. Press{' '}
                <b>
                    <em>Ctrl + 1</em>
                </b>{' '}
                to put first match at clipboard.{' '}
                <b>
                    <em>Ctrl + 2</em>
                </b>{' '}
                for second and so on. Same applies for double click.
            </p>
            <p>
                Press{' '}
                <b>
                    <em>{shortcutDisplay(settings.allShortcuts.openCcv)}</em>
                </b>{' '}
                to display main window. Next time, main window will be hidden at startup.
            </p>
        </Dialog>
    );
}
