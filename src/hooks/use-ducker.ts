import useSWR from "swr";
import { getDuckerConfig, patchDuckerConfig } from "@/services/cmds";
import type { Config } from "@/types";

export const useDucker = () => {
    const { data: config, mutate: mutateConfig } = useSWR(
        "getDuckerConfig",
        async () => {
        const config = await getDuckerConfig();
        return config;
        },
    );

    const patchDucker = async (config: Config) => {
        await patchDuckerConfig(config);
        mutateConfig();
    };

    return {
        config,
        patchDucker,
        mutateConfig
    }

};