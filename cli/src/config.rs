use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;

// Storing user selections while interating with mopro cli
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub(crate) target_adaptors: Vec<String>,
    pub(crate) target_platforms: Vec<String>,
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
