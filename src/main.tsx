// import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'


if (import.meta.env.PROD) {
    document.addEventListener('contextmenu', (e) => {
        e.preventDefault();
    });
}

createRoot(document.getElementById('root')!).render(
  // <StrictMode>
    <App />
  // </StrictMode> 
)
