// src-tauri/src/path_utils.rs
use tauri::{AppHandle, Manager};
use std::path::PathBuf;

const CONFIG_FILENAME: &str = "config.toml";

/// Returns `<config_dir>/<app-name>/config.toml`
pub fn external_config_path(handle: &AppHandle) -> Result<PathBuf, String> {
    let app_name = handle.package_info().name.clone();
    Ok(handle
        .path()
        .config_dir()
        .map_err(|e| format!("config_dir error: {e}"))?
        .join(app_name)
        .join(CONFIG_FILENAME))
}

/// Returns `<app_data_dir>/media`
pub fn media_dir(handle: &AppHandle) -> Result<PathBuf, String> {
    let app_name = handle.package_info().name.clone();
    Ok(handle
        .path()
        .data_dir()
        .map_err(|e| format!("app_data_dir error: {e}"))?
        .join(app_name)
        .join("media"))
}

