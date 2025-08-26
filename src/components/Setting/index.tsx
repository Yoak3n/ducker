
import type { FC } from "react"
import { SidebarProvider } from "@/components/ui/sidebar"

import { AppSidebar } from "./AppSidebar"
import SettingItems from "./SettingItems"

import "./index.css"


const SettingLayout: FC = () => {
  return (
    <div className="setting">
      <SidebarProvider defaultOpen={false} className="text-lg">
        <AppSidebar />
        <div className="setting-layout">
          <SettingItems />
        </div>
      </SidebarProvider>
    </div>

  )
}

export default SettingLayout;