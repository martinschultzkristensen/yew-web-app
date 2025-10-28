//src-tauri/src/lib.rs
mod commands;
use commands::*;
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use http::response::Builder as ResponseBuilder; // <-- there are often other builders in scope. Therefore rename to avoid ambiguity.
use tauri::http::{Request, Response};
use tauri::{Manager, Runtime};
use tauri_plugin_log::{Target, TargetKind};
use toml;
pub mod path_utils;

//const CONFIG_PATH: &str = "resources/config.toml";

pub struct ConfigError(String);
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConfigError: {}", self.0)
    }
}
impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigError: {}", self.0)
    }
}
impl std::error::Error for ConfigError {}

impl From<String> for ConfigError {
    fn from(err: String) -> Self {
        ConfigError(err)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ConfigDancer {
    pub name: String,
    pub image: String,
    pub strength: u8,
    pub flexibility: u8,
    pub in_choreography_nr: Vec<usize>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Dancers {
    pub list: Vec<ConfigDancer>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DemoVideoConfig {
    pub id: usize,
    pub url: String,
    pub loop_video: bool,
    pub title: String,
    pub description: Option<String>,
    pub duration: String,
    pub choreo_img: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct DemoVideos {
    pub list: Vec<DemoVideoConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ChoreoVideoConfig {
    pub id: usize,
    pub url: String,
    pub loop_video: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct ChoreoVideos {
    pub list: Vec<ChoreoVideoConfig>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct Config {
    pub dancers: Dancers,
    pub demo_videos: DemoVideos,
    pub choreo_videos: ChoreoVideos,
    pub intro_video: ChoreoVideoConfig,
    pub loadscreen_video: ChoreoVideoConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config_content = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
        let config: Config = toml::from_str(&config_content).map_err(|err| err.to_string())?;
        Ok(config)
    }
}

// now get's config from commands.rs Same code under comment: "Fall back to bundle config"
// #[tauri::command]
// fn get_config(handle: tauri::AppHandle) -> Result<Config, String> {
//     let resource_path = handle.path()
//     .resolve(CONFIG_PATH, BaseDirectory::Resource)
//     .map_err(|err| err.to_string())?;

//     Config::from_file(resource_path.to_str().ok_or("Invalid path")?)
//         .map_err(|err| err.to_string())

// }

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        //new section from here adding error output.
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),                     //<-- to terminal
                    Target::new(TargetKind::LogDir { file_name: None }), //<-- to an com.artfarm.danceomatic
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        //detailed logging in build
        .setup(|app| {
            // -----------------------------------------------------------------
            // 1️⃣ Log the resource directory (only for debugging – you can
            //    comment this out later)
            // -----------------------------------------------------------------
            if let Ok(res_dir) = app.path().resource_dir() {
                log::info!("📁 Resource dir: {:?}", res_dir);
                if let Ok(entries) = std::fs::read_dir(&res_dir) {
                    log::info!("📁 Resource dir contents:");
                    for entry in entries.flatten() {
                        log::info!("  - {:?}", entry.path());
                    }
                }
            }

            // -----------------------------------------------------------------
            // 2️⃣ Ensure the **media** folder exists exactly once.
            //    `media_dir()` returns the full path (<data_dir>/<app‑name>/media).
            // -----------------------------------------------------------------
            let media_path = path_utils::media_dir(&app.handle()).map_err(|e| {
                log::error!("❌ Could not compute media dir: {e}");
                e
            })?;

            // Create the folder if it isn’t there yet.
            if !media_path.exists() {
                std::fs::create_dir_all(&media_path).map_err(|e| {
                    log::error!("❌ Failed to create media dir {media_path:?}: {e}");
                    e
                })?;
                log::info!("🗂️ Created media folder at {:?}", media_path);
            } else {
                log::info!("📁 Media dir already exists: {:?}", media_path);
            }

            // Optional: list its current contents (helpful while developing)
            if let Ok(entries) = std::fs::read_dir(&media_path) {
                log::info!("📁 Media dir contents:");
                for entry in entries.flatten() {
                    log::info!("  - {:?}", entry.path());
                }
            }

            // -----------------------------------------------------------------
            // 3️⃣ Ensure the **config** file exists (or create a default one).
            //    `external_config_path()` gives us <config_dir>/<app‑name>/config.toml.
            // -----------------------------------------------------------------
            let cfg_path = path_utils::external_config_path(&app.handle()).map_err(|e| {
                log::error!("❌ Could not compute config path: {e}");
                e
            })?;

            // If the file is missing, copy the bundled default.
            if !cfg_path.exists() {
                // The default lives in the resources folder (read‑only).
                let default_res = app
                    .path()
                    .resource_dir()
                    .map_err(|e| format!("resource_dir error: {e}"))?
                    .join("config.toml"); // <-- adjust if you placed it elsewhere

                std::fs::copy(&default_res, &cfg_path).map_err(|e| {
                    log::error!("❌ Failed to copy default config: {e}");
                    e
                })?;
                log::info!("📝 Created default config at {:?}", cfg_path);
            } else {
                log::info!("📄 Config file already exists: {:?}", cfg_path);
            }

            // -----------------------------------------------------------------
            // 4️⃣ Final log – the app is ready to run.
            // -----------------------------------------------------------------
            log::info!("🔧 App setup complete");
            Ok(())
        })
        .register_uri_scheme_protocol("media", move |app, request| {
    let media_root = match path_utils::media_dir(&app.app_handle()) {
        Ok(p) => p,
        Err(e) => {
            log::error!("⚠️ Could not resolve media dir: {e}");
            return Response::builder().status(500).body(Vec::new()).unwrap();
        }
    };

    let uri = request.uri().to_string();
    let rel_path = uri.trim_start_matches("media://").trim_end_matches('/');
    let full_path: PathBuf = media_root.join(rel_path);

    log::info!("🔍 media:// request → {:?}", full_path);

    let mut file = match File::open(&full_path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("❌ Could not open file: {e}");
            return Response::builder().status(404).body(Vec::new()).unwrap();
        }
    };

    let metadata = file.metadata().unwrap();
    let file_size = metadata.len();
    let mime = mime_guess::from_path(&full_path).first_or_octet_stream();

    // Parse range header
    let range_header = request
        .headers()
        .get("range")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let (start, end, status) = if range_header.starts_with("bytes=") {
        let bytes = &range_header["bytes=".len()..];
        let mut parts = bytes.split('-');
        let start = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0);
        let end = parts.next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(file_size - 1);
        if start >= file_size {
            return Response::builder()
                .status(416)
                .header("Content-Range", format!("bytes */{file_size}"))
                .body(Vec::new())
                .unwrap();
        }
        (start, end.min(file_size - 1), 206)
    } else {
        (0, file_size - 1, 200)
    };

    let chunk_size = end - start + 1;
    let mut buffer = Vec::with_capacity(chunk_size as usize);
    file.seek(SeekFrom::Start(start)).unwrap();
    file.take(chunk_size).read_to_end(&mut buffer).unwrap();

    let mut resp = Response::builder()
        .status(status)
        .header("Accept-Ranges", "bytes")
        .header("Content-Type", mime.as_ref())
        .header("Content-Length", chunk_size.to_string());

    if status == 206 {
        resp = resp.header("Content-Range", format!("bytes {start}-{end}/{file_size}"));
    }

    resp.body(buffer).unwrap()
})

        .invoke_handler(tauri::generate_handler![
            get_config,
            debug_paths,
            reset_config_to_default,
            import_video,
            import_images,
            resolve_media_path,
            // get_video_path,
            // load_video,
            get_image_path,
            select_video_file,
            select_img_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
