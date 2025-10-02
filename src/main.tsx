// import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { getConfig } from './api/index.ts';
import { initializeLanguage, changeLanguage } from '@/services/i18n.ts';
import { listen } from '@tauri-apps/api/event';

if (import.meta.env.PROD) {
    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    });
}

const loadConfig = async () => {
    const config = await getConfig();
    config.language && changeLanguage(config.language);
}

(async () => {
    await initializeLanguage();
    await loadConfig();
    listen('config_updated', async (event) => {
        console.log('config_updated:', event);
        await loadConfig();
    });
    createRoot(document.getElementById('root')!).render(
      // <StrictMode>
        <App />
      // </StrictMode> 
    );
})();
