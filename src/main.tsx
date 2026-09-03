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

function applyTheme(theme?: string) {
    const root = document.documentElement;
    if (!theme || theme === 'system') {
        const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        root.classList.toggle('dark', isDark);
    } else {
        root.classList.toggle('dark', theme === 'dark');
    }
}

const loadConfig = async () => {
    const config = await getConfig();
    config.language && changeLanguage(config.language);
    applyTheme(config.theme);
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
