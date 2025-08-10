import {ContextMenuItem} from "@/components/ui/context-menu"
import { invoke } from "@tauri-apps/api/core"

const toggleMainWindow = async () => {
  const result = await invoke("toggle_main_window");
  console.log(result);
}
const toggleDashboardWindow = async () => {
  const result = await invoke("toggle_dashboard_window");
  console.log(result);
}



export default function ContextItems() {
  return (
    <>
      <ContextMenuItem onClick={toggleDashboardWindow}>
        打开任务面板
      </ContextMenuItem>
      <ContextMenuItem>
        打开Action面板
      </ContextMenuItem>
      <ContextMenuItem onClick={toggleMainWindow}>
        关闭Live2D
      </ContextMenuItem>
    </>
  )
}
