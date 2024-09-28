// import React from "react";
import ReactDOM from 'react-dom/client';
import { PrimeReactProvider } from 'primereact/api';
import App from './App.tsx';
import AppWrapper from '../common/wrapper/AppWrapper.tsx';

ReactDOM.createRoot(document.getElementById('root')!).render(
    // <React.StrictMode>
    <PrimeReactProvider>
        <AppWrapper>
            <App />
        </AppWrapper>
    </PrimeReactProvider>,
    // </React.StrictMode>,
);
