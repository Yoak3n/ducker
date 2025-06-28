import { invoke } from '@tauri-apps/api/core';
import {type Action, type CreateActionData, type UpdateActionData } from '../../store/types';
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
    create_action,
    update_action,
    delete_action,
    get_all_actions
}