use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

const SUPABASE_RESOURCE_PATH: &str = "resources/supabase.toml";
const SESSION_FILENAME: &str = "machine-session.json";
const REFRESH_MARGIN_SECONDS: u64 = 120;

#[derive(Debug, Clone, Deserialize)]
struct SupabaseConfig {
    supabase_url: String,
    publishable_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MachineSession {
    access_token: String,
    refresh_token: String,
    expires_at: u64,
}

#[derive(Debug, Deserialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,

    #[serde(default)]
    expires_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineManifest {
    pub schema_version: u32,
    pub generated_at: String,
    pub machine: ManifestMachine,
    pub machine_media: ManifestMachineMedia,
    pub choreographies: Vec<ManifestChoreography>,
    pub dancers: Vec<ManifestDancer>,
    pub choreography_dancers: Vec<ManifestChoreographyDancer>,
    pub files: Vec<ManifestFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestMachine {
    pub id: String,
    pub display_name: String,
    pub location: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestMachineMedia {
    pub intro_video_path: Option<String>,
    pub load_video_path: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestChoreography {
    pub id: String,
    pub display_order: u32,
    pub title: String,
    pub duration_seconds: u32,
    pub description: String,
    pub image_path: String,
    pub demo_video_path: String,
    pub choreo_video_path: String,
    pub status: String,
    pub visibility: String,
    pub updated_at: String,
    pub approved_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestDancer {
    pub id: String,
    pub name: String,
    pub image_path: Option<String>,
    pub strength: u8,
    pub flexibility: u8,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestChoreographyDancer {
    pub choreography_id: String,
    pub dancer_id: String,
    pub sort_order: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestFile {
    pub bucket: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MachineConnectionResult {
    pub machine_id: String,
    pub display_name: String,
    pub location: Option<String>,
    pub session_expires_at: u64,
    pub choreography_count: usize,
    pub file_count: usize,
}

fn unix_time_seconds() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| format!("System clock error: {error}"))
}

fn create_http_client() -> Result<Client, String> {
    Client::builder()
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(45))
        .user_agent("DanceOmatic-Machine/0.1")
        .build()
        .map_err(|error| format!("Failed to create HTTP client: {error}"))
}

fn load_supabase_config(handle: &AppHandle) -> Result<SupabaseConfig, String> {
    let resource_path = handle
        .path()
        .resolve(SUPABASE_RESOURCE_PATH, BaseDirectory::Resource)
        .map_err(|error| format!("Failed to resolve Supabase config: {error}"))?;

    let contents = fs::read_to_string(&resource_path).map_err(|error| {
        format!(
            "Failed to read Supabase config at {}: {error}",
            resource_path.display()
        )
    })?;

    let mut config: SupabaseConfig = toml::from_str(&contents)
        .map_err(|error| format!("Invalid resources/supabase.toml: {error}"))?;

    config.supabase_url = config.supabase_url.trim().trim_end_matches('/').to_string();
    config.publishable_key = config.publishable_key.trim().to_string();

    if config.supabase_url.is_empty() {
        return Err("supabase_url is missing in resources/supabase.toml".to_string());
    }

    if !config.supabase_url.starts_with("https://") {
        return Err("supabase_url must start with https://".to_string());
    }

    if config.publishable_key.is_empty() {
        return Err("publishable_key is missing in resources/supabase.toml".to_string());
    }

    if !config.publishable_key.starts_with("sb_publishable_") {
        return Err("publishable_key must be a Supabase sb_publishable_ key".to_string());
    }

    Ok(config)
}

fn session_path(handle: &AppHandle) -> Result<PathBuf, String> {
    let app_name = handle.package_info().name.clone();

    let session_directory = handle
        .path()
        .config_dir()
        .map_err(|error| format!("Failed to resolve config directory: {error}"))?
        .join(app_name);

    fs::create_dir_all(&session_directory).map_err(|error| {
        format!(
            "Failed to create session directory {}: {error}",
            session_directory.display()
        )
    })?;

    Ok(session_directory.join(SESSION_FILENAME))
}

fn load_session(handle: &AppHandle) -> Result<MachineSession, String> {
    let path = session_path(handle)?;

    let contents = fs::read_to_string(&path).map_err(|error| {
        format!(
            "Machine is not activated or the session cannot be read at {}: {error}",
            path.display()
        )
    })?;

    serde_json::from_str(&contents)
        .map_err(|error| format!("Invalid machine session file: {error}"))
}

fn save_session(handle: &AppHandle, session: &MachineSession) -> Result<(), String> {
    let path = session_path(handle)?;

    let contents = serde_json::to_vec_pretty(session)
        .map_err(|error| format!("Failed to serialize machine session: {error}"))?;

    fs::write(&path, contents)
        .map_err(|error| format!("Failed to save machine session: {error}"))?;

    // On Linux/macOS, only the current OS user may read the token file.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, permissions)
            .map_err(|error| format!("Failed to protect machine session file: {error}"))?;
    }

    Ok(())
}

fn session_from_auth_response(response: AuthResponse) -> Result<MachineSession, String> {
    let now = unix_time_seconds()?;

    Ok(MachineSession {
        access_token: response.access_token,
        refresh_token: response.refresh_token,
        expires_at: response
            .expires_at
            .unwrap_or_else(|| now.saturating_add(response.expires_in)),
    })
}

async fn parse_auth_response(
    context: &str,
    response: reqwest::Response,
) -> Result<MachineSession, String> {
    let status = response.status();

    let body = response
        .text()
        .await
        .map_err(|error| format!("{context}: failed to read response: {error}"))?;

    if !status.is_success() {
        return Err(format!("{context} failed with HTTP {status}: {body}"));
    }

    let auth_response: AuthResponse = serde_json::from_str(&body)
        .map_err(|error| format!("{context}: invalid authentication response: {error}"))?;

    session_from_auth_response(auth_response)
}

async fn sign_in_with_password(
    config: &SupabaseConfig,
    email: &str,
    password: &str,
) -> Result<MachineSession, String> {
    let client = create_http_client()?;

    let url = format!("{}/auth/v1/token?grant_type=password", config.supabase_url);

    let response = client
        .post(url)
        .header("apikey", &config.publishable_key)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password
        }))
        .send()
        .await
        .map_err(|error| format!("Machine login request failed: {error}"))?;

    parse_auth_response("Machine login", response).await
}

async fn refresh_session(
    config: &SupabaseConfig,
    old_session: &MachineSession,
) -> Result<MachineSession, String> {
    let client = create_http_client()?;

    let url = format!(
        "{}/auth/v1/token?grant_type=refresh_token",
        config.supabase_url
    );

    let response = client
        .post(url)
        .header("apikey", &config.publishable_key)
        .header("Content-Type", "application/json")
        .json(&json!({
            "refresh_token": old_session.refresh_token
        }))
        .send()
        .await
        .map_err(|error| format!("Session refresh request failed: {error}"))?;

    parse_auth_response("Session refresh", response).await
}

async fn load_or_refresh_session(
    handle: &AppHandle,
    config: &SupabaseConfig,
) -> Result<MachineSession, String> {
    let old_session = load_session(handle)?;
    let now = unix_time_seconds()?;

    if old_session.expires_at > now.saturating_add(REFRESH_MARGIN_SECONDS) {
        return Ok(old_session);
    }

    match refresh_session(config, &old_session).await {
        Ok(new_session) => {
            save_session(handle, &new_session)?;
            Ok(new_session)
        }
        Err(refresh_error) if old_session.expires_at > now => {
            log::warn!(
                "Could not refresh machine session early; using current token until expiry: {}",
                refresh_error
            );
            Ok(old_session)
        }
        Err(refresh_error) => Err(format!(
            "Machine session has expired and could not be refreshed: {refresh_error}"
        )),
    }
}

async fn fetch_manifest_from_supabase(
    config: &SupabaseConfig,
    session: &MachineSession,
) -> Result<MachineManifest, String> {
    let client = create_http_client()?;

    let url = format!(
        "{}/rest/v1/rpc/get_machine_sync_manifest",
        config.supabase_url
    );

    let response = client
        .post(url)
        .header("apikey", &config.publishable_key)
        .bearer_auth(&session.access_token)
        .header("Content-Type", "application/json")
        .json(&json!({}))
        .send()
        .await
        .map_err(|error| format!("Manifest request failed: {error}"))?;

    let status = response.status();

    let body = response
        .text()
        .await
        .map_err(|error| format!("Failed to read manifest response: {error}"))?;

    if !status.is_success() {
        return Err(format!(
            "Manifest request failed with HTTP {status}: {body}"
        ));
    }

    serde_json::from_str::<MachineManifest>(&body)
        .map_err(|error| format!("Invalid machine manifest: {error}. Response: {body}"))
}

fn connection_result(
    manifest: &MachineManifest,
    session: &MachineSession,
) -> MachineConnectionResult {
    MachineConnectionResult {
        machine_id: manifest.machine.id.clone(),
        display_name: manifest.machine.display_name.clone(),
        location: manifest.machine.location.clone(),
        session_expires_at: session.expires_at,
        choreography_count: manifest.choreographies.len(),
        file_count: manifest.files.len(),
    }
}

#[tauri::command]
pub async fn activate_machine(
    handle: AppHandle,
    email: String,
    password: String,
) -> Result<MachineConnectionResult, String> {
    let email = email.trim();

    if email.is_empty() {
        return Err("Machine email is required".to_string());
    }

    if password.is_empty() {
        return Err("Machine password is required".to_string());
    }

    let config = load_supabase_config(&handle)?;

    // Password exists only in memory during this request.
    let session = sign_in_with_password(&config, email, &password).await?;

    // Do not save a session unless the account is a valid linked machine.
    let manifest = fetch_manifest_from_supabase(&config, &session).await?;

    save_session(&handle, &session)?;

    log::info!("Machine activation completed for {}", manifest.machine.id);

    Ok(connection_result(&manifest, &session))
}

#[tauri::command]
pub async fn check_machine_connection(
    handle: AppHandle,
) -> Result<MachineConnectionResult, String> {
    let config = load_supabase_config(&handle)?;
    let session = load_or_refresh_session(&handle, &config).await?;
    let manifest = fetch_manifest_from_supabase(&config, &session).await?;

    Ok(connection_result(&manifest, &session))
}

#[tauri::command]
pub async fn fetch_machine_manifest(handle: AppHandle) -> Result<MachineManifest, String> {
    let config = load_supabase_config(&handle)?;
    let session = load_or_refresh_session(&handle, &config).await?;

    fetch_manifest_from_supabase(&config, &session).await
}

#[tauri::command]
pub fn clear_machine_session(handle: AppHandle) -> Result<bool, String> {
    let path = session_path(&handle)?;

    if !path.exists() {
        return Ok(false);
    }

    fs::remove_file(&path).map_err(|error| format!("Failed to remove machine session: {error}"))?;

    Ok(true)
}
