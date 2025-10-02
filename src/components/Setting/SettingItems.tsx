import { useEffect, type FC } from 'react'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { emit } from '@tauri-apps/api/event';

import { Switch } from "@/components/ui/switch"
import { Label } from '@/components/ui/label'

import { useConfigStore } from '@/store'
import SettingItem from './SettingItem'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select';
import { useI18n } from '@/hooks/use-i18n';
import type { Config } from '@/types';


const SettingItems: FC = () => {
    const fetchConfig = useConfigStore(state=>state.fetchConfig)
    const silent_launch = useConfigStore(state=>state.silent_launch)
    const auto_launch = useConfigStore(state=>state.enable_auto_launch)
    const setConfigStateInStore = useConfigStore(state=>state.setConfig)
    const language = useConfigStore(state=>state.language)
    // Zustand 的 useConfigStore(state => state.enable_auto_launch) 只有在组件重新渲染时才会拿到新值。
    // 需要手动获取新值
    useEffect(() => {
        const fetchData = async ()=> {
            await fetchConfig();
            const latest=  useConfigStore.getState().enable_auto_launch;
            const enabled = await isEnabled();
            if (enabled !== latest) {
                console.warn("Auto launch state out of sync, correcting in store:", enabled)
                if (latest){
                    setConfigStateInStore({ enable_auto_launch: true });
                }
            }
        }
        fetchData();
    }, [])
    
    const {t, supportedLanguagesMap} = useI18n()

    const handleConfigUpdate = async (config: Config) => {
        setConfigStateInStore(config);
        emit('config_updated', config);
    }

    return (
        <>
            <SettingItem id="general" title="General">
                <div className="flex gap-32">
                    <div className="flex items-center space-x-2">
                        <Switch id="run-on-start" checked={auto_launch} onCheckedChange={(v) => {
                            // v is the new state of the switch
                            if (!v) {
                                disable().then(() => handleConfigUpdate({ enable_auto_launch: false }))
                            } else {
                                enable().then(() => handleConfigUpdate({enable_auto_launch: true }))
                            }
                        }} />
                        <Label htmlFor="run-on-start">{t("Run on System Start")}</Label>
                    </div>
                    <div className="flex items-center space-x-2">
                        <Switch id="slient-start" checked={silent_launch} onCheckedChange={(v) => {
                            if (!v) {
                                handleConfigUpdate({ silent_launch: false })
                            } else {
                                handleConfigUpdate({ silent_launch: true })
                            }
                        }} />
                        <Label htmlFor="slient-start">{t("Slient Launch")}</Label>
                    </div>
                </div>
            </SettingItem>
            < SettingItem id="appearance" title="Appearance">
                <div className="flex gap-32">
                    <div className="flex items-center space-x-2">
                        <Label htmlFor="language">{t("Select Language")}</Label>
                        <Select 
                            value={language} 
                            defaultValue="zh" 
                            onValueChange={(v) => {
                                handleConfigUpdate({ language: v })
                            }}
                            >
                            <SelectTrigger>
                                <SelectValue placeholder={t("Language")} />
                            </SelectTrigger>
                            <SelectContent>
                                {Object.entries(supportedLanguagesMap).map(([key, value]) => (
                                    <SelectItem key={key} value={key}>{value}</SelectItem>
                                ))}
                            </SelectContent>
                        </Select>

                    </div>
                </div>
            </SettingItem>
        </>
    )
}

export default SettingItems