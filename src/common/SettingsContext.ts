import { createContext } from 'react';
import { Settings } from '../common/contract';

const defaultSettings: Settings = {
    allShortcuts: { openCcv: { altKey: false, ctrlKey: false, shiftKey: false, metaKey: false, code: undefined } },
    theme: 'Light',
    version: '',
    notifications: undefined,
};
export const SettingsContext = createContext<Settings>(defaultSettings);
