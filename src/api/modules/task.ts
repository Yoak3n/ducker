import type { Task, TaskData } from "@/types";
import { invoke } from "@tauri-apps/api/core";


async function create_task(task: TaskData) {
    try {
        const result = await invoke("create_task", { task });
        return result;
    } catch (err) {
        console.log(err);
    }
}

async function update_task(id: string, task: TaskData) {
    const result = await invoke("update_task", { id, task });
    return result;
}

async function delete_task(id: String) {
    const result = await invoke("delete_task", { id });
    return result;
}

async function get_task(id: string) {
    const task = await invoke("get_task", { id });
    return task;
}

async function get_all_tasks() {
    const tasks: Task[] = await invoke("get_all_tasks");
    return tasks;
}

async function get_tasks_by_date_range(start_date: Date, end_date: Date) {
    const tasks: Task[] = await invoke("get_tasks_by_date_range", { start_date, end_date });
    return tasks;
}

async function get_tasks_by_status(completed: boolean = false) {
    const tasks: Task[] = await invoke("get_tasks_by_status", { completed });
    return tasks;
}

async function get_tasks(ids: string[]) {
    const tasks: Task[] = await invoke("get_task", { ids });
    return tasks;
}


export { 
    create_task, 
    get_all_tasks,
    get_task,
    get_tasks_by_date_range,
    get_tasks_by_status,
    get_tasks,
    update_task,
    delete_task
};
