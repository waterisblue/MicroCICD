use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: i32,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_content = fs::read_to_string("./config/setting.toml")
        .expect("Unable to read config file");

    toml::de::from_str(&config_content)
        .expect("Unable to parse config file")
});

pub static TASK: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let config_content = fs::read_to_string("./config/task.properties")
        .expect("Unable to read config file");

    let map = config_content.lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, "=");
            let key = parts.next()?.trim().to_string();
            let value = parts.next()?.trim().to_string();
            Some((key, value))
        })
        .collect::<HashMap<String, String>>();

    map
});