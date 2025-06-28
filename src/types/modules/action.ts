export interface Action {
    id: string;
    name: string;
    desc: string;
    wait: number;
    type: 'exec_command' | 'open_url' | 'open_file' | 'open_dir';
    retry?: number;
    timeout?: number;
    command?: string;
    url?: string;
    args?: string[];
    createdAt?: string;
    updatedAt?: string;
}

export interface ActionType {
    value: string;
    label: string;
    icon?: string;
}

export type ActionTypeValue = 'exec_command' | 'open_url' | 'open_file' | 'open_dir';

export const actionTypes:ActionType[] = [
  { value: 'all', label: '全部类型', icon: 'apps' },
  { value: 'file', label: '文件操作', icon: 'folder' },
  { value: 'directory', label: '目录操作', icon: 'folder_open' },
  { value: 'command', label: '命令执行', icon: 'terminal' },
  { value: 'url', label: '网页链接', icon: 'link' }
];