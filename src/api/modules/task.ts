import type { Task, TaskData, PeriodicTask, PeriodicTaskData } from "@/types";
import { invoke } from "@tauri-apps/api/core";


async function create_task(task: TaskData) {
    try {
        const result = await invoke<string>("create_task", { task });
        return result;
    } catch (err) {
        console.log(err);
    }
}

async function update_task(id: string, task: TaskData): Promise<Task> {
    const result = await invoke<Task>("update_task", { id, task });
    return result;
}

async function update_task_status(id: string, completed: boolean): Promise<Task> {
    const result = await invoke<Task>("update_task_status", { id, completed });
    return result;
}

async function delete_task(id: string): Promise<void> {
    const result = await invoke<void>("delete_task", { id });
    return result;
}

async function get_task(id: string): Promise<Task> {
    const task = await invoke<Task>("get_task", { id });
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

// 周期任务相关函数
async function create_periodic_task(task: PeriodicTaskData): Promise<string> {
    const result = await invoke<string>("create_periodic_task", { task });
    return result;
}

async function update_periodic_task(id: string, task: PeriodicTaskData): Promise<PeriodicTask> {
    const result = await invoke<PeriodicTask>("update_periodic_task", { id, task });
    return result;
}

async function update_periodic_task_last_run(id: string): Promise<void> {
    const result = await invoke<void>("update_periodic_task_last_run", { id });
    return result;
}

async function delete_periodic_task(id: string): Promise<void> {
    const result = await invoke<void>("delete_periodic_task", { id });
    return result;
}

async function get_enabled_periodic_tasks(): Promise<PeriodicTask[]> {
    const tasks = await invoke<PeriodicTask[]>("get_enabled_periodic_tasks");
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
    update_task_status,
    delete_task,
    // 周期任务相关导出
    create_periodic_task,
    update_periodic_task,
    update_periodic_task_last_run,
    delete_periodic_task,
    get_enabled_periodic_tasks
};
