import { useEffect, useState, type FC } from 'react'
import { Switch } from "@/components/ui/switch"
import { Label } from '../ui/label'
import SettingItem from './SettingItem'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
const SettingItems: FC = () => {
    const [runOnStartEnabled, setRunOnStartEnabled] = useState(false)
    useEffect(() => {
        isEnabled().then(setRunOnStartEnabled)
    }, [])
    return (
        <>
            <SettingItem id="general" title="General">
                <div className="flex items-center space-x-2">
                    <Switch id="run-on-start" checked={runOnStartEnabled} onCheckedChange={()=>{
                        if (runOnStartEnabled) {
                            disable().then(() => {
                                setRunOnStartEnabled(false)
                            })
                        } else {
                            enable().then(() => {
                                setRunOnStartEnabled(true)
                            })
                        }
                    }}/>
                    <Label htmlFor="run-on-start">随系统启动</Label>
                </div>
                <div className="flex items-center space-x-2">
                    <Switch id="run-on-start" checked={runOnStartEnabled} onCheckedChange={()=>{
                        if (runOnStartEnabled) {
                            disable().then(() => {
                                setRunOnStartEnabled(false)
                            })
                        } else {
                            enable().then(() => {
                                setRunOnStartEnabled(true)
                            })
                        }
                    }}/>
                    <Label htmlFor="run-on-start">随系统启动</Label>
                </div>
            </SettingItem>

        </>
    )
}

export default SettingItems