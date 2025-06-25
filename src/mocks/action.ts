import type{ Action } from '@/types/modules/action';

export const mockActions: Action[] = [
  {
    id: '1',
    name: '打开文件',
    description: '使用默认程序打开指定文件',
    type: 'file',
    wait: 0,
    cmd: 'start',
    args: ['"{{filepath}}"']
  },
  {
    id: '2',
    name: '打开目录',
    description: '在文件管理器中打开指定目录',
    type: 'directory',
    wait: 0,
    cmd: 'explorer',
    args: ['"{{dirpath}}"']
  },
  {
    id: '3',
    name: '运行命令',
    description: '在命令行中执行指定命令',
    type: 'command',
    wait: 1000,
    cmd: '{{command}}',
    args: ['{{args}}']
  },
  {
    id: '4',
    name: '打开网页',
    description: '在默认浏览器中打开指定URL',
    type: 'url',
    wait: 0,
    cmd: 'start',
    args: ['"{{url}}"']
  },
  {
    id: '5',
    name: '发送邮件',
    description: '打开默认邮件客户端发送邮件',
    type: 'email',
    wait: 0,
    cmd: 'start',
    args: ['mailto:{{email}}?subject={{subject}}&body={{body}}']
  },
  {
    id: '6',
    name: '截图保存',
    description: '截取屏幕并保存到指定位置',
    type: 'command',
    wait: 2000,
    cmd: 'powershell',
    args: ['-Command', 'Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait("%{PRTSC}")']
  },
  {
    id: '7',
    name: '复制文件',
    description: '将文件复制到指定位置',
    type: 'file',
    wait: 500,
    cmd: 'copy',
    args: ['"{{source}}"', '"{{destination}}"']
  },
  {
    id: '8',
    name: '创建文件夹',
    description: '在指定位置创建新文件夹',
    type: 'directory',
    wait: 0,
    cmd: 'mkdir',
    args: ['"{{dirname}}"']
  },
  {
    id: '9',
    name: '启动应用',
    description: '启动指定的应用程序',
    type: 'command',
    wait: 3000,
    cmd: '{{apppath}}',
    args: []
  },
  {
    id: '10',
    name: '系统通知',
    description: '显示系统通知消息',
    type: 'command',
    wait: 0,
    cmd: 'powershell',
    args: ['-Command', 'New-BurntToastNotification -Text "{{title}}", "{{message}}"']
  }
];

export const actionTypes = [
  { value: 'all', label: '全部类型', icon: 'apps' },
  { value: 'file', label: '文件操作', icon: 'folder' },
  { value: 'directory', label: '目录操作', icon: 'folder_open' },
  { value: 'command', label: '命令执行', icon: 'terminal' },
  { value: 'url', label: '网页链接', icon: 'link' },
  { value: 'email', label: '邮件发送', icon: 'email' }
];