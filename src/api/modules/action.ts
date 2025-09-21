import { invoke } from '@tauri-apps/api/core';
import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
} from '@tauri-apps/plugin-notification';


import type { Action, CreateActionData, UpdateActionData } from '@/types';
async function execute_actions(actions: Action[] | undefined) {
    try {
        if (!actions || actions.length === 0) {
            console.warn('No actions to execute');
            return;
        }
        await invoke('execute_actions', { actions });
        let permissionGranted = await isPermissionGranted();
        
        // If not we need to request it
        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }

        // Once permission has been granted we can send the notification
        if (permissionGranted) {
            sendNotification({ title: 'Action执行完成', body: actions.map(a => a.name).join(',') });
        }
    } catch (err) {
        console.error('Error executing actions:', err);
    }
}

async function execute_single_action(action: Action) {
    try {
        const result = await invoke<string>('execute_single_action', { action });
        return result;
    } catch (err) {
        console.error('Error executing single action:', err);
        throw err;
    }
}

async function create_action(action: CreateActionData): Promise<Action> {
    return await invoke<Action>('create_action', { action });
}

async function update_action(id: string, action: UpdateActionData): Promise<Action> {
    return await invoke<Action>('update_action', { id, action });
}

async function delete_action(id: string): Promise<void> {
    return await invoke<void>('delete_action', { id });
}

async function get_all_actions() {
    try {
        const actions = await invoke<Action[]>('get_all_actions');
        return actions;
    } catch (err) {
        console.error('Error getting actions:', err);
        throw err;
    }
}

export {
    execute_actions,
    execute_single_action,
    create_action,
    update_action,
    delete_action,
    get_all_actions
}