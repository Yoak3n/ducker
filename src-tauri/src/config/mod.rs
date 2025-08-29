use crate::utils::{
    dirs,
    help::{read_yaml, save_yaml},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub enable_auto_launch: Option<bool>,
    pub silent_launch: Option<bool>,
}

impl Config {
    pub fn global() -> &'static std::sync::Mutex<Config> {
        static INSTANCE: std::sync::OnceLock<std::sync::Mutex<Config>> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(|| std::sync::Mutex::new(Self::new()))
    }

    fn new() -> Self {
        match dirs::config_path() {
            Ok(path) => match read_yaml::<Config>(&path) {
                Ok(config) => {
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
        }
    }

    pub fn data(&self) -> &Self {
        self
        // match dirs::config_path() {
        //     Ok(path) => match read_yaml::<Config>(&path) {
        //         Ok(config) => {
        //             println!("read config success: {:?}", config);
        //             config
        //         }
        //         Err(err) => {
        //             println!("read config error: {err}");
        //             log::error!(target: "app", "{err}");
        //             Self::template()
        //         }
        //     },
        //     Err(err) => {
        //         log::error!(target: "app", "{err}");
        //         println!("read config error: {err}");
        //         Self::template()
        //     }
        // }
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
    }
}
