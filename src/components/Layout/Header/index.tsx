import { getCurrentWindow} from '@tauri-apps/api/window';

import { closeWindow, minimizeWindow } from '@/api';
import "./index.css"
import type React from 'react';


export default function Header() {
    const handleClose = async() => await closeWindow(getCurrentWindow().label);
    const handleMinimize = async() => await minimizeWindow(getCurrentWindow().label);
    const handleMaximize = async() => {
        const currentWindow = getCurrentWindow();
        if (await currentWindow.isMaximized()) {
            await currentWindow.unmaximize();
        } else {
            await currentWindow.maximize();
            await currentWindow.setFocus();
        }
        
    }


    const handleDoubleClick = (e:React.MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();
        // 阻止整个标题栏的双击最大化行为。尽管没什么作用
    }

    return (
        <header className="header" data-tauri-drag-region onDoubleClick={handleDoubleClick}>
            <div className="header-title">
                Ducker
            </div>
            <div className="header-operation" onDoubleClick={handleDoubleClick}>
                <button onClick={handleMinimize}>
                    <span className="material-symbols-outlined">
                        remove
                    </span>
                </button>
                <button onClick={handleMaximize}>
                    <span className="material-symbols-outlined">
                        fullscreen
                    </span>
                </button>
                <button onClick={handleClose}>
                    <span className="material-symbols-outlined">
                        close
                    </span>
                </button>
            </div>
        </header>
    )
}