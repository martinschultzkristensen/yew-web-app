//src-tauri/src/lib.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod config_manager;

pub fn run() {
    tauri::Builder::default()
        // Register the commands
        .invoke_handler(tauri::generate_handler![
            config_manager::get_config,
            config_manager::save_config,
            config_manager::get_asset_path,
            config_manager::list_assets,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
