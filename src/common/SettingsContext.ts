import { createContext } from 'react';
import { Settings } from '../api';

const defaultSettings: Settings = {
    allShortcuts: { openCcv: { altKey: false, ctrlKey: false, shiftKey: false, metaKey: false, code: undefined } },
    theme: 'Light',
};
const SettingsContext = createContext<Settings>(defaultSettings);

export default SettingsContext;
