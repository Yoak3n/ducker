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

export interface McpStatus {
    enabled: boolean
    path_registered: boolean
    config_recorded: boolean
    exe_dir: string
    mcp_exe_exists: boolean
}

async function getMcpStatus(): Promise<McpStatus> {
    return await invoke<McpStatus>('get_mcp_status');
}

async function registerMcpPath(): Promise<McpStatus> {
    return await invoke<McpStatus>('register_mcp_path');
}

async function unregisterMcpPath(): Promise<McpStatus> {
    return await invoke<McpStatus>('unregister_mcp_path');
}

export { getConfig, setConfig, saveConfigToFile, getMcpStatus, registerMcpPath, unregisterMcpPath };
