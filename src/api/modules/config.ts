import { invoke } from '@tauri-apps/api/core';
import {type Config} from '@/types'

async function getConfig(): Promise<Config> { 
    return await invoke<Config>('get_config');
}

async function setConfig(config: Config): Promise<Config> {
    return await invoke('update_config', { config });
}

async function saveConfigToFile(): Promise<void> {
    return await invoke('get_config');
}

export { getConfig, setConfig, saveConfigToFile };