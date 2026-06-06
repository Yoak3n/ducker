import { getCurrentWindow } from '@tauri-apps/api/window';
import { Minus, Square, X } from 'lucide-react';

import { closeWindow, minimizeWindow } from '@/api';
import "./index.css"
import type React from 'react';


export default function Header() {
    const handleClose = async () => await closeWindow(getCurrentWindow().label);
    const handleMinimize = async () => await minimizeWindow(getCurrentWindow().label);
    const handleMaximize = async () => {
        const currentWindow = getCurrentWindow();
        if (await currentWindow.isMaximized()) {
            await currentWindow.unmaximize();
        } else {
            await currentWindow.maximize();
            await currentWindow.setFocus();
        }
    }

    const handleDoubleClick = (e: React.MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();
    }

    return (
        <header className="header" data-tauri-drag-region onDoubleClick={handleDoubleClick}>
            <div className="header-title">
                Ducker
            </div>
            <div className="header-operation" onDoubleClick={handleDoubleClick}>
                <button onClick={handleMinimize}>
                    <Minus size={16} />
                </button>
                <button onClick={handleMaximize}>
                    <Square size={14} />
                </button>
                <button onClick={handleClose}>
                    <X size={16} />
                </button>
            </div>
        </header>
    )
}