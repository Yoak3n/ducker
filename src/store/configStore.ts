import { create } from 'zustand';
import { devtools, subscribeWithSelector } from 'zustand/middleware';
import * as configApi from '../api/modules/config';
import type { ConfigState } from './types';


interface ConfigStore extends ConfigState{
    fetchConfig: () => Promise<boolean>;
    setConfig: (newConfig: Partial<ConfigState>) => void;

    clearError: () => void;
    reset: () => void;
}

const initialState: ConfigState = {
    loading: false,
    error: null,
    enable_auto_launch: false,
    silent_launch: false,
    filters: {}
}

export const useConfigStore = create<ConfigStore>()(
    subscribeWithSelector(
        devtools(
            (set, get) => ({
            ...initialState,
            fetchConfig: async ():Promise<boolean> => {
                set({ loading: true, error: null },false);
                try {
                    const config = await configApi.getConfig();
                    set({ enable_auto_launch: config.enable_auto_launch,silent_launch: config.silent_launch,  loading: false },false);
                } catch (error:any) {
                    set({ error: error.message || 'Failed to fetch config', loading: false });
                }
                return true;
            },
            setConfig: (newConfig) => {
                console.log("setting config in store:", newConfig)
                const updatedConfig = { ...get(), ...newConfig };
                console.log("updating config in store:", updatedConfig)
                set(updatedConfig);
                configApi.setConfig(updatedConfig).catch((error) => {
                    set({ error: error.message || 'Failed to save config' });
                });
            },

            clearError: () => set({ error: null }),
            reset: () => set(initialState),
        }))
    )
);