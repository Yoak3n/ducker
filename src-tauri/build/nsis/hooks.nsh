; ducker NSIS 安装钩子（tauri.conf.json -> bundle.windows.nsis.installerHooks 引用）
;
; 背景：dsh 的 MCP 插件会以守护方式保持 ducker-mcp.exe 存活——进程被杀后
; 约 20 秒内自动重启。因此仅 taskkill 不够（写文件前就可能被重新拉起锁定）。
;
; 策略（利用 Windows 特性：运行中的 exe 不能被覆写，但可以被重命名）：
;   1. taskkill 结束 GUI 与 MCP 进程
;   2. 若 exe 仍存在（守护已重新拉起，或 taskkill 失败），把旧 exe 改名为 *.old，
;      让出目标路径——安装器即可写入新文件；改名的旧映像不影响运行中的进程
;   3. *.old 会在下次升级时被清理，也可手动删除

!macro _ducker_clear_path _exe
  nsExec::ExecToLog 'taskkill /IM "${_exe}" /F'
  Pop $0
  Sleep 500
  IfFileExists "$INSTDIR\${_exe}" 0 +3
    Rename "$INSTDIR\${_exe}" "$INSTDIR\${_exe}.old"
!macroend

!macro NSIS_HOOK_PREINSTALL
  !insertmacro _ducker_clear_path "ducker-mcp.exe"
  !insertmacro _ducker_clear_path "ducker.exe"
  Sleep 500
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  !insertmacro _ducker_clear_path "ducker-mcp.exe"
  !insertmacro _ducker_clear_path "ducker.exe"
  Sleep 500
  ; 卸载时顺带清理历史遗留的 .old 备份
  Delete "$INSTDIR\ducker-mcp.exe.old"
  Delete "$INSTDIR\ducker.exe.old"
!macroend
