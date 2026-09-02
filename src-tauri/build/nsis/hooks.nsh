; ducker NSIS 安装钩子（tauri.conf.json -> bundle.windows.nsis.installerHooks 引用）
;
; 背景：dsh 的 MCP 插件会以守护方式保持 ducker-mcp.exe 存活（被杀后数秒内自动重启），
; 运行中的 exe 被写锁锁定，安装器覆盖会报“无法打开要写入的文件”。
; 因此在解压覆盖前先结束两个二进制的运行实例。
; dsh 检测到子进程退出后会自行重启 ducker-mcp（命中的是新版文件），无需额外处理。

!macro NSIS_HOOK_PREINSTALL
  nsExec::ExecToLog 'taskkill /IM ducker-mcp.exe /F'
  nsExec::ExecToLog 'taskkill /IM ducker.exe /F'
  ; 留出句柄释放时间，避免 taskkill 返回后写锁尚未完全释放
  Sleep 800
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  nsExec::ExecToLog 'taskkill /IM ducker-mcp.exe /F'
  nsExec::ExecToLog 'taskkill /IM ducker.exe /F'
  Sleep 800
!macroend
