import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { registerAll, unregisterAll } from '@tauri-apps/api/globalShortcut';
import { CopyItem } from '../api';

export function useListenGlobalEvents() {
    async function listen() {
        // await registerAll(['CommandOrControl+Shift+C', 'Ctrl+Alt+F12'], (shortcut) => {
        //     console.log(`Shortcut ${shortcut} triggered`);
        // });
    
        await registerAll(['SHIFT+`'], async () => {
            await invoke<CopyItem[]>('show_window');
        });
    
        return unregisterAll;
    }
    
    useEffect(() => {
        const promise = listen();
        return () => {
            promise.then(c => c());
        }
    }, []);
}
