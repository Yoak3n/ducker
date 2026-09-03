import { useEffect, useState, type FC } from 'react'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { emit } from '@tauri-apps/api/event';
import { toast } from 'sonner';

import { Switch } from "@/components/ui/switch"
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'

import { useConfigStore } from '@/store'
import { getMcpStatus, registerMcpPath, unregisterMcpPath, type McpStatus } from '@/api/modules/config';
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
    const theme = useConfigStore(state=>state.theme)
    const enable_mcp = useConfigStore(state=>state.enable_mcp)
    const [mcpStatus, setMcpStatus] = useState<McpStatus | null>(null)

    const refreshMcpStatus = async () => {
        try {
            setMcpStatus(await getMcpStatus())
        } catch (error) {
            console.warn('Failed to fetch MCP status:', error)
        }
    }

    useEffect(() => {
        refreshMcpStatus()
    }, [])

    // MCP 客户端配置片段：已注册 PATH 用短命令，否则回退完整路径
    const isWindows = navigator.platform.toLowerCase().includes('win');
    const mcpCommand = mcpStatus?.path_registered
        ? 'ducker-mcp'
        : mcpStatus?.exe_dir
            ? `${mcpStatus.exe_dir}${isWindows ? '\\' : '/'}${isWindows ? 'ducker-mcp.exe' : 'ducker-mcp'}`
            : 'ducker-mcp';
    const mcpConfigSnippet = JSON.stringify(
        { mcpServers: { ducker: { command: mcpCommand, args: [], env: {} } } },
        null,
        2
    );
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
                    enable().then(() => handleConfigUpdate({ enable_auto_launch: true }))
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
            <SettingItem id="mcp" title="MCP Service">
                <div className="flex flex-col gap-4">
                    <div className="flex items-center space-x-2">
                        <Switch id="enable-mcp" checked={!!enable_mcp} onCheckedChange={(v) => {
                            handleConfigUpdate({ enable_mcp: v })
                        }} />
                        <Label htmlFor="enable-mcp">{t("Enable MCP Service")}</Label>
                    </div>
                    <div className="flex items-center gap-4 flex-wrap">
                        <Button
                            id="register-mcp-path"
                            size="sm"
                            variant="outline"
                            disabled={!mcpStatus}
                            onClick={async () => {
                                try {
                                    const next = mcpStatus?.path_registered
                                        ? await unregisterMcpPath()
                                        : await registerMcpPath();
                                    setMcpStatus(next);
                                    toast.success(next.path_registered
                                        ? t("PATH Registered")
                                        : t("PATH Removed"));
                                } catch (error) {
                                    toast.error(String(error));
                                }
                            }}
                        >
                            {mcpStatus?.path_registered ? t("Remove from PATH") : t("Add to PATH")}
                        </Button>
                        <div className="flex flex-col gap-1 text-sm text-muted-foreground">
                            <Label>{t("MCP Status")}: {mcpStatus
                                ? `${mcpStatus.path_registered ? t("PATH Registered") : t("PATH Not Registered")}${mcpStatus.mcp_exe_exists ? '' : ' · ' + t("ducker-mcp.exe not found")}`
                                : t("Loading")}</Label>
                            <Label className="font-normal opacity-70">{mcpStatus?.exe_dir ?? ''}</Label>
                        </div>
                    </div>
                    <div className="flex flex-col gap-2">
                        <div className="flex items-center justify-between">
                            <Label>{t("MCP Config")}</Label>
                            <Button
                                size="sm"
                                variant="ghost"
                                disabled={!mcpStatus}
                                onClick={async () => {
                                    try {
                                        await navigator.clipboard.writeText(mcpConfigSnippet);
                                        toast.success(t("Copied to clipboard"));
                                    } catch (error) {
                                        toast.error(String(error));
                                    }
                                }}
                            >
                                {t("Copy Config")}
                            </Button>
                        </div>
                        <pre className="text-xs bg-muted rounded-md p-3 overflow-x-auto whitespace-pre">{mcpConfigSnippet}</pre>
                        <p className="text-xs text-muted-foreground">{t("MCP Config Hint")}</p>
                    </div>
                    <p className="text-sm text-muted-foreground">
                        {t("MCP Service Description")}
                    </p>
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
                                {Object.entries(supportedLanguagesMap).map(([key, label]) => (
                                    <SelectItem key={key} value={key}>{label}</SelectItem>
                                ))}
                            </SelectContent>
                        </Select>

                    </div>
                    <div className="flex items-center space-x-2">
                        <Label htmlFor="theme">{t("Theme")}</Label>
                        <Select
                            value={theme || "system"}
                            defaultValue="system"
                            onValueChange={(v) => {
                                handleConfigUpdate({ theme: v })
                            }}
                            >
                            <SelectTrigger>
                                <SelectValue placeholder={t("Theme")} />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value="light">{t("Theme Light")}</SelectItem>
                                <SelectItem value="dark">{t("Theme Dark")}</SelectItem>
                                <SelectItem value="system">{t("Theme System")}</SelectItem>
                            </SelectContent>
                        </Select>
                    </div>
                </div>
            </SettingItem>
        </>
    )
}

export default SettingItems