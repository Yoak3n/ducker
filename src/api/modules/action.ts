import { invoke } from '@tauri-apps/api/core';
import {type Action } from '@/types/index';
async function execute_actions(actions: Action[]|undefined) {
    try {
        if (!actions || actions.length === 0) {
            console.warn('No actions to execute');
            return;
        }
        await invoke('execute_actions', { actions });
    }catch (err) {
        console.error('Error executing actions:', err);
    }
}

async function create_action(action:Action){
    try{
        await invoke('create_action',{action})
    }catch(err){
        console.error('Error creating action:', err);
    }
}


export {
    execute_actions,
    create_action
}