import { getCurrentWindow} from '@tauri-apps/api/window';
import "./index.css"

export default function Header() {
    const handleClose = async() => await getCurrentWindow().close();
    const handleMinimize = async() => await getCurrentWindow().minimize()
    const handleMaximize = async() => {
        const currentWindow = await getCurrentWindow();
        if (await currentWindow.isMaximized()) {
            await currentWindow.unmaximize();
        } else {
            await currentWindow.maximize();
            await currentWindow.setFocus();
        }
        
    }


    return (
        <header className="header" data-tauri-drag-region>
            <div className="header-title">
                Ducker
            </div>
            <div className="header-operation">
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