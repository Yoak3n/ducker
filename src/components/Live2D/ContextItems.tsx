import { ContextMenuItem } from "@/components/ui/context-menu"
import { invoke } from "@tauri-apps/api/core"

const toggleDashboardWindow = async () => {
  const result = await invoke("toggle_dashboard_window");
  console.log(result);
}

interface Props {
  isSettingOpen: boolean;
  setIsSettingOpen: (isOpen: boolean) => void;
}

export default function ContextItems({ setIsSettingOpen }: Props) {
  return (
    <>
      <ContextMenuItem onClick={toggleDashboardWindow}>
        打开任务面板
      </ContextMenuItem>
      <ContextMenuItem onClick={toggleDashboardWindow}>
        打开Action面板
      </ContextMenuItem>
      <ContextMenuItem onClick={() => setIsSettingOpen(true)}>
        设置Live2D
      </ContextMenuItem>
    </>
  )
}
