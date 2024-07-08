// import React from "react";
import ReactDOM from 'react-dom/client';
import { PrimeReactProvider } from 'primereact/api';
import App from './App.tsx';

import 'primereact/resources/themes/nano/theme.css';
import 'primeicons/primeicons.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
    // <React.StrictMode>
    <PrimeReactProvider>
        <App />
    </PrimeReactProvider>,
    // </React.StrictMode>,
);
