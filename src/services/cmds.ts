import type { Config } from "@/types";
import { invoke } from "@tauri-apps/api/core";


export async function getDuckerConfig(){
    return invoke("get_config");
}

export async function patchDuckerConfig(config: Config){
    return invoke("update_config", { config });
}