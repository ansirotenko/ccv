// import React from "react";
import ReactDOM from 'react-dom/client';
import logo from '../assets/logo256.png';

ReactDOM.createRoot(document.getElementById('root')!).render(
    // <React.StrictMode>
    <>
        <style>{'body { margin: 0}'}</style>
        <div className="splashscreen" style={{ backgroundColor: '#d0f0f7', textAlign: 'center', height: '100vh' }}>
            <img src={logo} />
        </div>
    </>,
    // </React.StrictMode>,
);
