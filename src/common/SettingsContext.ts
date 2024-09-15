import { createContext } from 'react';
import { Settings } from '../api';

const defaultSettings: Settings = {
    keybindings: { openCcv: '' },
    theme: 'Light',
};
const SettingsContext = createContext<Settings>(defaultSettings);

export default SettingsContext;
