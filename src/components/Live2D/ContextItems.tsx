import { ContextMenuItem } from "@/components/ui/context-menu"

import { toggleWindow } from "@/api";
import { useI18n } from "@/hooks/use-i18n";

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
  const { t } = useI18n();
  return (
    <>
      <ContextMenuItem onClick={toggleDashboardWindow}>
        {t("Open Dashboard")}
      </ContextMenuItem>
      <ContextMenuItem onClick={toggleActionWindow}>
        {t("Open Action")}
      </ContextMenuItem>
      <ContextMenuItem onClick={() => setIsSettingOpen(true)}>
        {t("Live2d Setting")}
      </ContextMenuItem>
    </>
  )
}
