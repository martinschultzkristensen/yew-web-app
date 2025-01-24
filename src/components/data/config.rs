use gloo::console::log; 
use serde::{Serialize, Deserialize};
use std::fs;
use std::toml;

#[derive(Debug, Deserialize)]
struct Config {
    key: String,
    value: i32,
}

fn configread() -> Result<(), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("/static/config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    println!("Key: {}, Value: {}", config.key, config.value);
    Ok(())
}