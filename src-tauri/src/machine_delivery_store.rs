use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

const STORE_DIRECTORY: &str = "machine-delivery";
const STAGING_DIRECTORY: &str = "staging";
const DEPLOYMENTS_DIRECTORY: &str = "deployments";
const FACTORY_DIRECTORY: &str = "factory";
const STATE_FILENAME: &str = "state.json";
const STATE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalDeliveryReference {
    pub deployment_id: String,
    pub version: i64,
    pub directory_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalDeliveryState {
    pub schema_version: u32,
    pub active: Option<LocalDeliveryReference>,
    pub previous: Option<LocalDeliveryReference>,
}

impl Default for LocalDeliveryState {
    fn default() -> Self {
        Self {
            schema_version: STATE_SCHEMA_VERSION,
            active: None,
            previous: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MachineDeliveryStorageInfo {
    pub root: String,
    pub staging: String,
    pub deployments: String,
    pub factory: String,
    pub state_file: String,
    pub state: LocalDeliveryState,
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn storage_root(handle: &AppHandle) -> Result<PathBuf, String> {
    let app_name = handle.package_info().name.clone();

    Ok(handle
        .path()
        .data_dir()
        .map_err(|error| format!("Failed to resolve application data directory: {error}"))?
        .join(app_name)
        .join(STORE_DIRECTORY))
}

fn ensure_directory(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path)
        .map_err(|error| format!("Failed to create directory {}: {error}", path.display()))
}

fn load_or_create_state(path: &Path) -> Result<LocalDeliveryState, String> {
    if path.exists() {
        let contents = fs::read_to_string(path)
            .map_err(|error| format!("Failed to read {}: {error}", path.display()))?;

        let state: LocalDeliveryState = serde_json::from_str(&contents)
            .map_err(|error| format!("Invalid local delivery state: {error}"))?;

        if state.schema_version != STATE_SCHEMA_VERSION {
            return Err(format!(
                "Unsupported local delivery state schema version {}",
                state.schema_version
            ));
        }

        return Ok(state);
    }

    let state = LocalDeliveryState::default();

    let contents = serde_json::to_vec_pretty(&state)
        .map_err(|error| format!("Failed to serialize local delivery state: {error}"))?;

    fs::write(path, contents)
        .map_err(|error| format!("Failed to create {}: {error}", path.display()))?;

    Ok(state)
}

#[tauri::command]
pub fn initialize_machine_delivery_storage(
    handle: AppHandle,
) -> Result<MachineDeliveryStorageInfo, String> {
    let root = storage_root(&handle)?;
    let staging = root.join(STAGING_DIRECTORY);
    let deployments = root.join(DEPLOYMENTS_DIRECTORY);
    let factory = root.join(FACTORY_DIRECTORY);
    let state_file = root.join(STATE_FILENAME);

    ensure_directory(&root)?;
    ensure_directory(&staging)?;
    ensure_directory(&deployments)?;
    ensure_directory(&factory)?;

    let state = load_or_create_state(&state_file)?;

    Ok(MachineDeliveryStorageInfo {
        root: path_string(&root),
        staging: path_string(&staging),
        deployments: path_string(&deployments),
        factory: path_string(&factory),
        state_file: path_string(&state_file),
        state,
    })
}
