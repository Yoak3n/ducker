use crate::{
    core::handle::Handle, logging, 
    module::auto_launch,
    utils::{
        dirs,
        help::{read_yaml, save_yaml},
        logging::Type,
    }
};

use anyhow::Result;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tauri_plugin_autostart::ManagerExt;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub enable_auto_launch: Option<bool>,
    pub silent_launch: Option<bool>,
    pub language: Option<String>,
}

impl Config {
    pub fn global() -> &'static Mutex<Config> {
        static INSTANCE: std::sync::OnceLock<Mutex<Config>> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(|| Mutex::new(Self::new()))
    }

    fn new() -> Self {
        match dirs::config_path() {
            Ok(path) => match read_yaml::<Config>(&path) {
                Ok(config) => {
                    let is_enable = config.enable_auto_launch.unwrap_or(false);
                    if let Err(err) = try_original_autostart_method(is_enable) {
                        logging!(error,Type::System,true, "{:?}", err);
                        auto_launch::enable_auto_launch(is_enable).unwrap();
                    }
                    println!("read config success: {:?}", config);
                    config
                }
                Err(err) => {
                    println!("read config error: {err}");
                    log::error!(target: "app", "{err}");
                    Self::template()
                }
            },
            Err(err) => {
                log::error!(target: "app", "{err}");
                println!("read config error: {err}");
                Self::template()
            }
        }
    }

    fn template() -> Self {
        Self {
            enable_auto_launch: Some(false),
            silent_launch: Some(false),
            language: Some("zh".to_string()),
        }
    }

    pub fn data(&self) -> &Self {
        self
    }

    pub fn save(&self) -> Result<()> {
        let path = dirs::config_path()?;
        save_yaml(&path, self, None)
    }

    pub fn patch_config(&mut self, other: Config) {
        macro_rules! patch {
            ($key: tt) => {
                if other.$key.is_some() {
                    self.$key = other.$key;
                }
            };
        }
        patch!(enable_auto_launch);
        patch!(silent_launch);
        patch!(language);
    }
}

fn try_original_autostart_method(is_enable: bool) ->Result<()>{
    
    let app_handle = if let Some(handle) = Handle::global().app_handle() {
        handle
    } else {
        logging!(info,Type::System,true, "try_original_autostart_method: app_handle is None");
        return Err(anyhow::anyhow!("app_handle is None"));
    };
    let autostart_manager = app_handle.autolaunch();
    let autostart = autostart_manager.is_enabled().unwrap_or(false);
    if is_enable != autostart {
        logging!(info,Type::System,true, "try_original_autostart_method: {:?}", is_enable);
        if is_enable {
            autostart_manager.enable()?;
        } else {
            autostart_manager.disable()?;
        }
    }
    Ok(())
}
