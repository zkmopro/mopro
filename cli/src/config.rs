use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use crate::init::adapter::Adapter;

// Storing user selections while iterating with mopro cli
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub(crate) build_mode: Option<String>,
    pub(crate) target_adapters: Option<HashSet<String>>,
    pub(crate) target_platforms: Option<HashSet<String>>,
    pub(crate) ios: Option<HashSet<String>>,
    pub(crate) android: Option<HashSet<String>>,
}

impl Config {
    pub fn adapter_eq(&self, adapter: Adapter) -> bool {
        self.target_adapters == Some(HashSet::from([String::from(adapter.as_str())]))
    }
    pub fn adapter_contains(&self, adapter: Adapter) -> bool {
        if let Some(adapters) = &self.target_adapters {
            adapters.contains(adapter.as_str())
        } else {
            false
        }
    }
}

pub fn read_config(file_path: &PathBuf) -> Result<Config> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn write_config(file_path: &PathBuf, config: &Config) -> Result<()> {
    let toml_string = toml::to_string_pretty(config)?;
    let mut file = File::create(file_path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
