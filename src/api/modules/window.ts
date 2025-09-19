import { invoke } from "@tauri-apps/api/core";


function toggleWindow(windowType: string) {
    return invoke("toggle_window", { windowType });
}

function showWindow(windowType: string, url?: string) {
    return invoke("show_window", { windowType, url });
}

function closeWindow(windowType: string) {
    return invoke("close_window", { windowType });
}

export { toggleWindow, showWindow, closeWindow };
