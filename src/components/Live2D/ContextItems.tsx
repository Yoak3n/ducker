import { ContextMenuItem } from "@/components/ui/context-menu"


import { toggleWindow } from "@/api";

const toggleDashboardWindow = async () => {
  const result = await toggleWindow("dashboard");
  console.log(result);
}

const toggleActionWindow = async () => {
  const result = await toggleWindow("action");
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
      <ContextMenuItem onClick={toggleActionWindow}>
        打开Action面板
      </ContextMenuItem>
      <ContextMenuItem onClick={() => setIsSettingOpen(true)}>
        设置Live2D
      </ContextMenuItem>
    </>
  )
}
