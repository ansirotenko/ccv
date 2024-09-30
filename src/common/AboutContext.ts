import { createContext } from 'react';
import { AboutData } from './contract';

export const AboutContext = createContext<AboutData | undefined>(undefined);
