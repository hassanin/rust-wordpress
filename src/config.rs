// use std::collections::HashMap;
// use std::env;
use serde::{Deserialize, Serialize};
use std::fs;
// Reads the config files

pub fn read_config() -> Result<ConfigSettings, String> {
    let filename = "./config/main.json";
    let contents = fs::read_to_string(filename).map_err(|err| {
        println!("Caught fatal error {:?}", err);
        err.to_string()
    })?;
    let val: ConfigSettings = serde_json::from_str(&contents).map_err(|err| err.to_string())?;
    Ok(val)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSettings {
    pub port: String,
    pub log_level: LogLevel,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "error")]
    Error,
}
