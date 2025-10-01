import { invoke } from "@tauri-apps/api/core";


function toggleWindow(windowType: string) {
    return invoke("toggle_window", { windowType });
}

function showWindow(windowType: string, url?: string) {
    return invoke("show_window", { windowType, url });
}

function minimizeWindow(windowType: string) {
    return invoke("minimize_window", { windowType });
}

function closeWindow(windowType: string) {
    return invoke("close_window", { windowType });
}

function destroyWindow(windowType: string) {
    return invoke("destroy_window", { windowType });
}

export { toggleWindow, showWindow, minimizeWindow, closeWindow, destroyWindow };
