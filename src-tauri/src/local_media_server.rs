use crate::machine_delivery_store::initialize_machine_delivery_storage;
use crate::path_utils::media_dir;
use axum::{routing::get, Router};
use std::path::PathBuf;
use tauri::AppHandle;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub const LOCAL_MEDIA_BASE_URL: &str = "http://127.0.0.1:17847";
const LOCAL_MEDIA_ADDRESS: &str = "127.0.0.1:17847";

pub fn start(handle: AppHandle) -> Result<(), String> {
    eprintln!("Starting local DanceOmatic media server...");

    let storage = initialize_machine_delivery_storage(handle.clone())?;
    let deployments_directory = PathBuf::from(storage.deployments);
    let media_directory = media_dir(&handle)?;

    // Bind porten med det samme, så vi straks opdager portfejl.
    let listener = std::net::TcpListener::bind(LOCAL_MEDIA_ADDRESS)
        .map_err(|error| {
            format!(
                "Could not bind local media server to {LOCAL_MEDIA_ADDRESS}: {error}"
            )
        })?;

    listener
        .set_nonblocking(true)
        .map_err(|error| format!("Could not configure local media server: {error}"))?;

    eprintln!("Local media port bound at http://{LOCAL_MEDIA_ADDRESS}");

    std::thread::Builder::new()
        .name("danceomatic-media-server".to_string())
        .spawn(move || {
            let runtime = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(error) => {
                    eprintln!("Could not create local media runtime: {error}");
                    return;
                }
            };

            runtime.block_on(async move {
                let listener = match tokio::net::TcpListener::from_std(listener) {
                    Ok(listener) => listener,
                    Err(error) => {
                        eprintln!("Could not create Tokio media listener: {error}");
                        return;
                    }
                };

                let router = Router::new()
                    .route("/health", get(|| async { "ok" }))
                    .nest_service(
                        "/delivery",
                        ServeDir::new(deployments_directory),
                    )
                    .nest_service(
                        "/media",
                        ServeDir::new(media_directory),
                    )
                    .layer(CorsLayer::permissive());

                eprintln!(
                    "Local media server listening on http://{LOCAL_MEDIA_ADDRESS}"
                );

                log::info!(
                    "Local media server listening on http://{LOCAL_MEDIA_ADDRESS}"
                );

                if let Err(error) = axum::serve(listener, router).await {
                    eprintln!("Local media server stopped: {error}");
                    log::error!(
                        "Local media server stopped with an error: {error}"
                    );
                }
            });
        })
        .map_err(|error| format!("Could not create local media server thread: {error}"))?;

    Ok(())
}
