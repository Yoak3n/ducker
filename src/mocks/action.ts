import type{ Action } from '@/types/modules/action';

export const mockActions: Action[] = [
  {
    id: '1',
    name: '打开文件',
    desc: '使用默认程序打开指定文件',
    type: 'file',
    wait: 0,
    command: 'start',
    args: ['"{{filepath}}"']
  },
  {
    id: '2',
    name: '打开目录',
    desc: '在文件管理器中打开指定目录',
    type: 'directory',
    wait: 0,
    command: 'explorer',
    args: ['"{{dirpath}}"']
  },
  {
    id: '3',
    name: '运行命令',
    desc: '在命令行中执行指定命令',
    type: 'command',
    wait: 1000,
    command: '{{command}}',
    args: ['{{args}}']
  },
  {
    id: '4',
    name: '打开网页',
    desc: '在默认浏览器中打开指定URL',
    type: 'url',
    wait: 0,
    command: 'start',
    args: ['"{{url}}"']
  },
  {
    id: '5',
    name: '截图保存',
    desc: '截取屏幕并保存到指定位置',
    type: 'command',
    wait: 2000,
    command: 'powershell',
    args: ['-Command', 'Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait("%{PRTSC}")']
  },
  {
    id: '6',
    name: '复制文件',
    desc: '将文件复制到指定位置',
    type: 'file',
    wait: 500,
    command: 'copy',
    args: ['"{{source}}"', '"{{destination}}"']
  },
  {
    id: '7',
    name: '创建文件夹',
    desc: '在指定位置创建新文件夹',
    type: 'directory',
    wait: 0,
    command: 'mkdir',
    args: ['"{{dirname}}"']
  },
  {
    id: '8',
    name: '启动应用',
    desc: '启动指定的应用程序',
    type: 'command',
    wait: 3000,
    command: '{{apppath}}',
    args: []
  },
  {
    id: '9',
    name: '系统通知',
    desc: '显示系统通知消息',
    type: 'command',
    wait: 0,
    command: 'powershell',
    args: ['-Command', 'New-BurntToastNotification -Text "{{title}}", "{{message}}"']
  }
];