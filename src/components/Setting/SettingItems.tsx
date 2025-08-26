import { useEffect, type FC } from 'react'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';

import { Switch } from "@/components/ui/switch"
import { Label } from '@/components/ui/label'

import { useConfigStore } from '@/store'
import SettingItem from './SettingItem'


const SettingItems: FC = () => {
    const fetchConfig = useConfigStore(state=>state.fetchConfig)
    const silent_launch = useConfigStore(state=>state.silent_launch)
    const auto_launch = useConfigStore(state=>state.enable_auto_launch)
    const setConfigStateInStore = useConfigStore(state=>state.setConfig)

    // Zustand 的 useConfigStore(state => state.enable_auto_launch) 只有在组件重新渲染时才会拿到新值。
    // 需要手动获取新值
    
    useEffect(() => {
        const fetchData = async ()=> {
        await fetchConfig();
        const latest=  useConfigStore.getState().enable_auto_launch;
        const enabled = await isEnabled();
        if (enabled !== latest) {
            console.warn("Auto launch state out of sync, correcting in store:", enabled)
            setConfigStateInStore({ enable_auto_launch: enabled });
        }
    }
        fetchData();
    }, [])
    
    return (
        <>
            <SettingItem id="general" title="General">
                <div className="flex gap-32">
                    <div className="flex items-center space-x-2">
                        <Switch id="run-on-start" checked={auto_launch} onCheckedChange={(v) => {
                            // v is the new state of the switch
                            if (!v) {
                                disable().then(() => setConfigStateInStore({ enable_auto_launch: false }))
                            } else {
                                enable().then(() => setConfigStateInStore({enable_auto_launch: true }))
                            }
                        }} />
                        <Label htmlFor="run-on-start">随系统启动</Label>
                    </div>
                    <div className="flex items-center space-x-2">
                        <Switch id="slient-start" checked={silent_launch} onCheckedChange={(v) => {
                            if (!v) {
                                setConfigStateInStore({ silent_launch: false })
                            } else {
                                setConfigStateInStore({ silent_launch: true })
                            }
                        }} />
                        <Label htmlFor="slient-start">静默启动</Label>
                    </div>
                </div>
            </SettingItem>
        </>
    )
}

export default SettingItems