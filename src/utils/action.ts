import type{ Action } from '@/types/index';
import { invoke } from '@tauri-apps/api/core';
export function exec_action(action: Action) {
    console.log(`Executing action: ${action.name}`);
    console.log(`Description: ${action.desc}`);
    console.log(`Type: ${action.type}`);
    console.log(`Wait time: ${action.wait}ms`);
    console.log(`Command: ${action.command}`);
    console.log(`Arguments: ${action.args?.join(', ')}`);
    
    // 模拟执行命令
    setTimeout(() => {
        console.log(`Action ${action.name} executed successfully.`);
    }, action.wait);
}


export async function open_file_select_dialog(file:boolean){
    const path:string = await invoke("select_file",{file})
    return path
}