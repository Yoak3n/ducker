import type{ Action } from '@/types/index';
import { log } from 'console';
export function exec_action(action: Action) {
    log(`Executing action: ${action.name}`);
    log(`Description: ${action.description}`);
    log(`Type: ${action.type}`);
    log(`Wait time: ${action.wait}ms`);
    log(`Command: ${action.cmd}`);
    log(`Arguments: ${action.args.join(', ')}`);
    
    // 模拟执行命令
    setTimeout(() => {
        log(`Action ${action.name} executed successfully.`);
    }, action.wait);
}