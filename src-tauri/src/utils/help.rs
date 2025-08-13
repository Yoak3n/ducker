use anyhow::{anyhow, bail, Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml::Mapping;
use std::{fs, path::PathBuf};

pub fn read_yaml<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    if !path.exists() {
        bail!("file not found \"{}\"", path.display());
    }

    let yaml_str = fs::read_to_string(path)
        .with_context(|| format!("failed to read the file \"{}\"", path.display()))?;

    serde_yaml::from_str::<T>(&yaml_str).with_context(|| {
        format!(
            "failed to read the file with yaml format \"{}\"",
            path.display()
        )
    })
}

pub fn read_mapping(path: &PathBuf) -> Result<Mapping> {
    if !path.exists() {
        bail!("file not found \"{}\"", path.display());
    }

    let yaml_str = fs::read_to_string(path)
        .with_context(|| format!("failed to read the file \"{}\"", path.display()))?;

    // YAML语法检查
    match serde_yaml::from_str::<serde_yaml::Value>(&yaml_str) {
        Ok(mut val) => {
            val.apply_merge()
                .with_context(|| format!("failed to apply merge \"{}\"", path.display()))?;

            Ok(val
                .as_mapping()
                .ok_or(anyhow!(
                    "failed to transform to yaml mapping \"{}\"",
                    path.display()
                ))?
                .to_owned())
        }
        Err(err) => {
            let error_msg = format!("YAML syntax error in {}: {}", path.display(), err);
            println!("{}", error_msg);

            crate::core::handle::Handle::notice_message(
                "config_validate::yaml_syntax_error",
                &error_msg,
            );

            bail!("YAML syntax error: {}", err)
        }
    }
}

pub fn save_yaml<T: Serialize>(path: &PathBuf, data: &T, prefix: Option<&str>) -> Result<()> {
    let data_str = serde_yaml::to_string(data)?;

    let yaml_str = match prefix {
        Some(prefix) => format!("{prefix}\n\n{data_str}"),
        None => data_str,
    };

    let path_str = path.as_os_str().to_string_lossy().to_string();
    fs::write(path, yaml_str.as_bytes())
        .with_context(|| format!("failed to save file \"{path_str}\""))
}

pub fn get_uid(prefix: &str) -> String {
    let id = random_string(11);
    format!("{prefix}{id}")
}

pub fn random_string(len: usize) -> String {
    use chrono::prelude::Local;
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    let dt = Local::now();
    let mut rng = rand::thread_rng();
    let random_str: String = (0..len).map(|_| rng.sample(Alphanumeric) as char).collect();

    let rand_ts = &dt.timestamp().to_string()[..6];
    let rand_result = format!("{}{}", random_str, rand_ts);
    rand_result
}
#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_random_string() {
        let random_str = random_string(10);
        println!("{}", random_str);
    }

    #[test]
    fn test_read_yaml() {
        let path = PathBuf::from("src/config.yaml");
        let config = read_yaml::<serde_yaml::Value>(&path).unwrap();
        println!("{:?}", config);
    }

    #[test]
    fn test_read_mapping() {
        let path = PathBuf::from("src/config.yaml");
        let config = read_mapping(&path).unwrap();
        println!("{:?}", config);
    }
}
