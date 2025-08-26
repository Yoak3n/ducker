
import type { FC } from "react"
import { SidebarProvider } from "@/components/ui/sidebar"

import { AppSidebar } from "./AppSidebar"
import SettingItems from "./SettingItems"

import "./index.css"


const SettingLayout: FC = () => {
  return (
    <SidebarProvider >
      <AppSidebar />
      <div className="setting-layout">
        <SettingItems />
      </div>
    </SidebarProvider>
  )
}

export default SettingLayout;