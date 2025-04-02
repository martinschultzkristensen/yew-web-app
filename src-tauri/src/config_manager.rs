//src-tauri/src/config_manager.rs
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

#[tauri::command]
pub fn get_config(app_handle: AppHandle) -> Result<String, String> {
    let app_dir = app_handle.path().app_config_dir()
    .map_err(|_| "Failed to get config dir".to_string())?;

Ok(app_dir.to_string_lossy().into_owned());
    
    // Create config directory if it doesn't exist
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    
    // Config file path
    let config_path = app_dir.join("config.toml");
    
    // If config doesn't exist, copy from resources
    if !config_path.exists() {
        let resource_path = app_handle.path().resource_dir()
        .map_err(|_| "Failed to find bundled config".to_string())?;
    Ok(resource_path.to_string_lossy().into_owned());
        
        fs::copy(&resource_path, &config_path).map_err(|e| e.to_string())?;
    }
    
    // Read and return config
    fs::read_to_string(config_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_config(app_handle: AppHandle, config_content: String) -> Result<(), String> {
    let app_dir = app_handle.path_resolver().app_config_dir()
        .ok_or("Failed to get config dir")?;
    
    // Create config directory if it doesn't exist
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    
    // Config file path
    let config_path = app_dir.join("config.toml");
    
    // Write config
    fs::write(config_path, config_content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_asset_path(app_handle: AppHandle, asset_name: String) -> Result<String, String> {
    let app_data_dir = app_handle.path_resolver().app_data_dir()
        .ok_or("Failed to get data dir")?;
    let assets_dir = app_data_dir.join("assets");
    
    // Create assets directory if it doesn't exist
    fs::create_dir_all(&assets_dir).map_err(|e| e.to_string())?;
    
    let asset_path = assets_dir.join(&asset_name);
    
    // If asset doesn't exist, copy from resources
    if !asset_path.exists() {
        let resource_path = app_handle.path_resolver()
            .resolve_resource(format!("static/{}", asset_name))
            .ok_or(format!("Failed to find bundled asset: {}", asset_name))?;
        
        fs::copy(&resource_path, &asset_path).map_err(|e| e.to_string())?;
    }
    
    // Return the path as a string
    asset_path.to_str()
        .ok_or("Failed to convert path to string".to_string())
        .map(|s| s.to_string())
}

#[tauri::command]
pub fn list_assets(app_handle: AppHandle) -> Result<Vec<String>, String> {
    let app_data_dir = app_handle.path_resolver().app_data_dir()
        .ok_or("Failed to get data dir")?;
    let assets_dir = app_data_dir.join("assets");
    
    // Create assets directory if it doesn't exist
    if !assets_dir.exists() {
        fs::create_dir_all(&assets_dir).map_err(|e| e.to_string())?;
    }
    
    let entries = fs::read_dir(&assets_dir).map_err(|e| e.to_string())?;
    let mut asset_names = Vec::new();
    
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                asset_names.push(file_name.to_string());
            }
        }
    }
    
    Ok(asset_names)
}