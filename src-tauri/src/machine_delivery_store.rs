use crate::supabase_sync::{
    authenticated_storage_client, fetch_latest_machine_delivery, report_machine_delivery_result,
    start_machine_delivery_download, MachineDelivery,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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

#[derive(Debug, Clone, Serialize)]
pub struct MachineDeliveryDownloadResult {
    pub deployment_id: String,
    pub version: i64,
    pub staging_directory: String,
    pub delivery_file: String,
    pub expected_files: usize,
    pub downloaded_files: usize,
    pub bytes_downloaded: u64,
    pub remote_status: String,
}

fn validate_storage_segment(segment: &str, description: &str) -> Result<(), String> {
    if segment.is_empty() {
        return Err(format!("{description} must not be empty"));
    }

    if segment == "." || segment == ".." {
        return Err(format!(
            "{description} contains an unsafe path segment: {segment}"
        ));
    }

    if segment
        .chars()
        .any(|character| matches!(character, '/' | '\\' | ':' | '\0'))
    {
        return Err(format!(
            "{description} contains an unsupported character: {segment}"
        ));
    }

    if segment.ends_with(' ') || segment.ends_with('.') {
        return Err(format!(
            "{description} must not end with a space or dot: {segment}"
        ));
    }

    Ok(())
}

fn validate_deployment_id(deployment_id: &str) -> Result<(), String> {
    if deployment_id.is_empty()
        || !deployment_id
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
    {
        return Err(format!(
            "Deployment ID cannot be used as a directory name: {deployment_id}"
        ));
    }

    Ok(())
}

fn staging_directory_name(delivery: &MachineDelivery) -> Result<String, String> {
    if delivery.version < 1 {
        return Err(format!(
            "Delivery version must be greater than zero, received {}",
            delivery.version
        ));
    }

    validate_deployment_id(&delivery.id)?;

    Ok(format!("version-{:020}-{}", delivery.version, delivery.id))
}

fn storage_destination(
    staging_directory: &Path,
    bucket: &str,
    object_path: &str,
) -> Result<PathBuf, String> {
    if bucket != bucket.trim() {
        return Err(format!(
            "Storage bucket contains leading or trailing whitespace: {bucket:?}"
        ));
    }

    if object_path != object_path.trim() {
        return Err(format!(
            "Storage object path contains leading or trailing whitespace: {object_path:?}"
        ));
    }

    validate_storage_segment(bucket, "Storage bucket")?;

    let mut destination = staging_directory.join("files").join(bucket);

    for segment in object_path.split('/') {
        validate_storage_segment(segment, "Storage object path")?;
        destination.push(segment);
    }

    Ok(destination)
}

async fn write_json_file<T>(path: &Path, value: &T) -> Result<(), String>
where
    T: Serialize + ?Sized,
{
    use tokio::io::AsyncWriteExt as _;

    let parent = path.parent().ok_or_else(|| {
        format!(
            "JSON destination has no parent directory: {}",
            path.display()
        )
    })?;

    tokio::fs::create_dir_all(parent).await.map_err(|error| {
        format!(
            "Failed to create JSON directory {}: {error}",
            parent.display()
        )
    })?;

    let contents = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("Failed to serialize {}: {error}", path.display()))?;

    let mut file = tokio::fs::File::create(path)
        .await
        .map_err(|error| format!("Failed to create {}: {error}", path.display()))?;

    file.write_all(&contents)
        .await
        .map_err(|error| format!("Failed to write {}: {error}", path.display()))?;

    file.flush()
        .await
        .map_err(|error| format!("Failed to flush {}: {error}", path.display()))?;

    file.sync_all()
        .await
        .map_err(|error| format!("Failed to synchronize {}: {error}", path.display()))?;

    Ok(())
}

async fn stage_machine_delivery(
    handle: &AppHandle,
    delivery: &MachineDelivery,
    remote_status: String,
) -> Result<MachineDeliveryDownloadResult, String> {
    if delivery.file_count != delivery.manifest.files.len() {
        return Err(format!(
            "Delivery file count mismatch: delivery says {} but manifest contains {}",
            delivery.file_count,
            delivery.manifest.files.len()
        ));
    }

    if delivery.choreography_count != delivery.manifest.choreographies.len() {
        return Err(format!(
            "Delivery choreography count mismatch: delivery says {} but manifest contains {}",
            delivery.choreography_count,
            delivery.manifest.choreographies.len()
        ));
    }

    let storage = initialize_machine_delivery_storage(handle.clone())?;
    let staging_root = PathBuf::from(storage.staging);
    let staging_directory = staging_root.join(staging_directory_name(delivery)?);
    let delivery_file = staging_directory.join("delivery.json");

    if staging_directory.exists() {
        tokio::fs::remove_dir_all(&staging_directory)
            .await
            .map_err(|error| {
                format!(
                    "Failed to remove old staging directory {}: {error}",
                    staging_directory.display()
                )
            })?;
    }

    tokio::fs::create_dir_all(&staging_directory)
        .await
        .map_err(|error| {
            format!(
                "Failed to create staging directory {}: {error}",
                staging_directory.display()
            )
        })?;

    let staging_result = async {
        write_json_file(&delivery_file, delivery).await?;

        let storage_client = authenticated_storage_client(handle).await?;
        let mut seen_files = HashSet::new();
        let mut downloaded_files = 0_usize;
        let mut bytes_downloaded = 0_u64;

        for manifest_file in &delivery.manifest.files {
            let unique_key = format!("{}\0{}", manifest_file.bucket, manifest_file.path);

            if !seen_files.insert(unique_key) {
                return Err(format!(
                    "Manifest contains duplicate Storage object: {}/{}",
                    manifest_file.bucket, manifest_file.path
                ));
            }

            let destination = storage_destination(
                &staging_directory,
                &manifest_file.bucket,
                &manifest_file.path,
            )?;

            let file_bytes = storage_client
                .download_object_to_file(&manifest_file.bucket, &manifest_file.path, &destination)
                .await?;

            bytes_downloaded = bytes_downloaded
                .checked_add(file_bytes)
                .ok_or_else(|| "Total downloaded byte count overflowed".to_string())?;

            downloaded_files += 1;
        }

        if downloaded_files != delivery.file_count {
            return Err(format!(
                "Incomplete staging download: expected {} files but downloaded {}",
                delivery.file_count, downloaded_files
            ));
        }

        Ok(MachineDeliveryDownloadResult {
            deployment_id: delivery.id.clone(),
            version: delivery.version,
            staging_directory: path_string(&staging_directory),
            delivery_file: path_string(&delivery_file),
            expected_files: delivery.file_count,
            downloaded_files,
            bytes_downloaded,
            remote_status,
        })
    }
    .await;

    if staging_result.is_err() {
        let _ = tokio::fs::remove_dir_all(&staging_directory).await;
    }

    staging_result
}

#[tauri::command]
pub async fn download_latest_machine_delivery_to_staging(
    handle: AppHandle,
) -> Result<MachineDeliveryDownloadResult, String> {
    let response = fetch_latest_machine_delivery(handle.clone()).await?;

    let delivery = response
        .latest_delivery
        .ok_or_else(|| "No machine delivery is available".to_string())?;

    let start_result = start_machine_delivery_download(handle.clone(), delivery.id.clone()).await?;

    if start_result.already_installed || start_result.status == "installed" {
        return Err(format!(
            "Machine delivery version {} is already installed",
            delivery.version
        ));
    }

    if start_result.status != "downloading" {
        return Err(format!(
            "Machine delivery could not enter downloading status; received {}",
            start_result.status
        ));
    }

    match stage_machine_delivery(&handle, &delivery, start_result.status).await {
        Ok(result) => Ok(result),
        Err(download_error) => {
            let report_result = report_machine_delivery_result(
                handle,
                delivery.id,
                false,
                Some(download_error.clone()),
            )
            .await;

            match report_result {
                Ok(_) => Err(download_error),
                Err(report_error) => Err(format!(
                    "{download_error}; additionally failed to report the download error: \
                     {report_error}"
                )),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MachineDeliveryActivationResult {
    pub deployment_id: String,
    pub version: i64,
    pub deployment_directory: String,
    pub state_file: String,
    pub active: LocalDeliveryReference,
    pub previous: Option<LocalDeliveryReference>,
    pub moved_from_staging: bool,
    pub already_active: bool,
    pub remote_reported: bool,
    pub remote_status: Option<String>,
    pub remote_report_error: Option<String>,
}

async fn read_delivery_file(path: &Path) -> Result<MachineDelivery, String> {
    let contents = tokio::fs::read(path)
        .await
        .map_err(|error| format!("Failed to read {}: {error}", path.display()))?;

    serde_json::from_slice(&contents)
        .map_err(|error| format!("Invalid delivery file {}: {error}", path.display()))
}

fn count_regular_files_recursively(directory: &Path) -> Result<usize, String> {
    let mut count = 0_usize;

    for entry_result in fs::read_dir(directory)
        .map_err(|error| format!("Failed to read directory {}: {error}", directory.display()))?
    {
        let entry = entry_result.map_err(|error| {
            format!(
                "Failed to read directory entry in {}: {error}",
                directory.display()
            )
        })?;

        let file_type = entry.file_type().map_err(|error| {
            format!(
                "Failed to inspect directory entry {}: {error}",
                entry.path().display()
            )
        })?;

        if file_type.is_symlink() {
            return Err(format!(
                "Symbolic links are not allowed in machine deliveries: {}",
                entry.path().display()
            ));
        }

        if file_type.is_dir() {
            count = count
                .checked_add(count_regular_files_recursively(&entry.path())?)
                .ok_or_else(|| "Local delivery file count overflowed".to_string())?;
        } else if file_type.is_file() {
            count = count
                .checked_add(1)
                .ok_or_else(|| "Local delivery file count overflowed".to_string())?;
        }
    }

    Ok(count)
}

fn validate_local_delivery_files(
    delivery_directory: &Path,
    delivery: &MachineDelivery,
) -> Result<(), String> {
    if delivery.file_count != delivery.manifest.files.len() {
        return Err(format!(
            "Delivery file count mismatch: delivery says {} but manifest contains {}",
            delivery.file_count,
            delivery.manifest.files.len()
        ));
    }

    if delivery.choreography_count != delivery.manifest.choreographies.len() {
        return Err(format!(
            "Delivery choreography count mismatch: delivery says {} but manifest contains {}",
            delivery.choreography_count,
            delivery.manifest.choreographies.len()
        ));
    }

    let files_directory = delivery_directory.join("files");

    if !files_directory.is_dir() {
        return Err(format!(
            "Local delivery files directory is missing: {}",
            files_directory.display()
        ));
    }

    let mut seen_files = HashSet::new();

    for manifest_file in &delivery.manifest.files {
        let unique_key = format!("{}\0{}", manifest_file.bucket, manifest_file.path);

        if !seen_files.insert(unique_key) {
            return Err(format!(
                "Manifest contains duplicate Storage object: {}/{}",
                manifest_file.bucket, manifest_file.path
            ));
        }

        let local_path = storage_destination(
            delivery_directory,
            &manifest_file.bucket,
            &manifest_file.path,
        )?;

        let metadata = fs::metadata(&local_path).map_err(|error| {
            format!(
                "Required local delivery file is missing {}: {error}",
                local_path.display()
            )
        })?;

        if !metadata.is_file() {
            return Err(format!(
                "Required local delivery path is not a file: {}",
                local_path.display()
            ));
        }

        if metadata.len() == 0 {
            return Err(format!(
                "Required local delivery file is empty: {}",
                local_path.display()
            ));
        }
    }

    let actual_file_count = count_regular_files_recursively(&files_directory)?;

    if actual_file_count != delivery.file_count {
        return Err(format!(
            "Local delivery contains an unexpected number of files: expected {} but found {}",
            delivery.file_count, actual_file_count
        ));
    }

    Ok(())
}

async fn validate_local_delivery(
    delivery_directory: &Path,
    expected_delivery: &MachineDelivery,
) -> Result<(), String> {
    let delivery_file = delivery_directory.join("delivery.json");
    let local_delivery = read_delivery_file(&delivery_file).await?;

    if local_delivery.id != expected_delivery.id {
        return Err(format!(
            "Local delivery ID {} does not match expected delivery ID {}",
            local_delivery.id, expected_delivery.id
        ));
    }

    if local_delivery.version != expected_delivery.version {
        return Err(format!(
            "Local delivery version {} does not match expected version {}",
            local_delivery.version, expected_delivery.version
        ));
    }

    if local_delivery.manifest_schema_version != expected_delivery.manifest_schema_version {
        return Err(format!(
            "Local manifest schema version {} does not match expected version {}",
            local_delivery.manifest_schema_version, expected_delivery.manifest_schema_version
        ));
    }

    if local_delivery.created_at != expected_delivery.created_at {
        return Err(format!(
            "Local delivery creation timestamp does not match the immutable delivery: {}",
            delivery_file.display()
        ));
    }

    if local_delivery.choreography_count != expected_delivery.choreography_count
        || local_delivery.file_count != expected_delivery.file_count
    {
        return Err(format!(
            "Local delivery counts do not match the immutable delivery: {}",
            delivery_file.display()
        ));
    }

    let expected_manifest = serde_json::to_value(&expected_delivery.manifest)
        .map_err(|error| format!("Failed to compare expected manifest: {error}"))?;

    let local_manifest = serde_json::to_value(&local_delivery.manifest)
        .map_err(|error| format!("Failed to compare local manifest: {error}"))?;

    if local_manifest != expected_manifest {
        return Err(format!(
            "Local manifest does not match the latest immutable delivery: {}",
            delivery_file.display()
        ));
    }

    validate_local_delivery_files(delivery_directory, &local_delivery)
}

#[cfg(windows)]
fn replace_file_atomically(source: &Path, destination: &Path) -> Result<(), String> {
    use std::os::windows::ffi::OsStrExt as _;

    const MOVEFILE_REPLACE_EXISTING: u32 = 0x0000_0001;
    const MOVEFILE_WRITE_THROUGH: u32 = 0x0000_0008;

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        #[link_name = "MoveFileExW"]
        fn move_file_ex_w(
            existing_file_name: *const u16,
            new_file_name: *const u16,
            flags: u32,
        ) -> i32;
    }

    let source_wide = source
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();

    let destination_wide = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();

    let succeeded = unsafe {
        move_file_ex_w(
            source_wide.as_ptr(),
            destination_wide.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };

    if succeeded == 0 {
        return Err(format!(
            "Failed to atomically replace {}: {}",
            destination.display(),
            std::io::Error::last_os_error()
        ));
    }

    Ok(())
}

#[cfg(not(windows))]
fn replace_file_atomically(source: &Path, destination: &Path) -> Result<(), String> {
    fs::rename(source, destination).map_err(|error| {
        format!(
            "Failed to atomically replace {}: {error}",
            destination.display()
        )
    })?;

    let parent = destination.parent().ok_or_else(|| {
        format!(
            "Local delivery state has no parent directory: {}",
            destination.display()
        )
    })?;

    fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| {
            format!(
                "Failed to synchronize state directory {}: {error}",
                parent.display()
            )
        })?;

    Ok(())
}

async fn write_state_atomically(
    state_file: &Path,
    state: &LocalDeliveryState,
) -> Result<(), String> {
    let state_filename = state_file
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            format!(
                "Local delivery state filename is invalid: {}",
                state_file.display()
            )
        })?;

    let temporary_file = state_file.with_file_name(format!("{state_filename}.next"));

    if temporary_file.exists() {
        tokio::fs::remove_file(&temporary_file)
            .await
            .map_err(|error| {
                format!(
                    "Failed to remove stale temporary state file {}: {error}",
                    temporary_file.display()
                )
            })?;
    }

    write_json_file(&temporary_file, state).await?;

    if let Err(error) = replace_file_atomically(&temporary_file, state_file) {
        let _ = tokio::fs::remove_file(&temporary_file).await;
        return Err(error);
    }

    Ok(())
}

#[tauri::command]
pub async fn activate_latest_staged_machine_delivery(
    handle: AppHandle,
) -> Result<MachineDeliveryActivationResult, String> {
    let response = fetch_latest_machine_delivery(handle.clone()).await?;

    let delivery = response
        .latest_delivery
        .ok_or_else(|| "No machine delivery is available for activation".to_string())?;

    let storage = initialize_machine_delivery_storage(handle.clone())?;

    let staging_root = PathBuf::from(&storage.staging);
    let deployments_root = PathBuf::from(&storage.deployments);
    let state_file = PathBuf::from(&storage.state_file);

    let directory_name = staging_directory_name(&delivery)?;
    let staging_directory = staging_root.join(&directory_name);
    let deployment_directory = deployments_root.join(&directory_name);

    let staging_exists = staging_directory.exists();
    let deployment_exists = deployment_directory.exists();

    if staging_exists && deployment_exists {
        return Err(format!(
            "Delivery exists in both staging and deployments: {}",
            directory_name
        ));
    }

    if !staging_exists && !deployment_exists {
        return Err(format!(
            "No complete local copy exists for machine delivery version {}",
            delivery.version
        ));
    }

    let mut moved_from_staging = false;

    if staging_exists {
        validate_local_delivery(&staging_directory, &delivery).await?;

        tokio::fs::rename(&staging_directory, &deployment_directory)
            .await
            .map_err(|error| {
                format!(
                    "Failed to move delivery from staging {} to deployments {}: {error}",
                    staging_directory.display(),
                    deployment_directory.display()
                )
            })?;

        moved_from_staging = true;
    }

    validate_local_delivery(&deployment_directory, &delivery).await?;

    let active_reference = LocalDeliveryReference {
        deployment_id: delivery.id.clone(),
        version: delivery.version,
        directory_name,
    };

    let mut new_state = storage.state;

    let already_active = new_state.active.as_ref().is_some_and(|active| {
        active.deployment_id == active_reference.deployment_id
            && active.version == active_reference.version
            && active.directory_name == active_reference.directory_name
    });

    if !already_active {
        new_state.previous = new_state.active.take();
        new_state.active = Some(active_reference.clone());

        write_state_atomically(&state_file, &new_state).await?;
    }

    let previous = new_state.previous.clone();

    let report_result =
        report_machine_delivery_result(handle, delivery.id.clone(), true, None).await;

    let (remote_reported, remote_status, remote_report_error) = match report_result {
        Ok(result) if result.status == "installed" || result.already_installed => {
            (true, Some(result.status), None)
        }
        Ok(result) => (
            false,
            Some(result.status.clone()),
            Some(format!(
                "Supabase returned unexpected delivery status {}",
                result.status
            )),
        ),
        Err(error) => (false, None, Some(error)),
    };

    Ok(MachineDeliveryActivationResult {
        deployment_id: delivery.id,
        version: delivery.version,
        deployment_directory: path_string(&deployment_directory),
        state_file: path_string(&state_file),
        active: active_reference,
        previous,
        moved_from_staging,
        already_active,
        remote_reported,
        remote_status,
        remote_report_error,
    })
}
