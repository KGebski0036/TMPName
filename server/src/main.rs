mod routes;
mod utils;
use anyhow::Result;
use axum::{routing::get, serve, Extension, Router};
use std::{
    env,
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tower_http::services::ServeDir;

use routes::{
    get_metadata_all::get_metadata_all, get_package_metadata::get_package_metadata, index::index,
};
use utils::{builder::build_metadata, watcher::watch_directory};

#[tokio::main]
async fn main() -> Result<()> {
    let repo_dir = PathBuf::from(env::var("REPO_DIR").unwrap_or(String::from("./repo")));
    let static_dir = PathBuf::from(env::var("TEMPLATE_DIR").unwrap()).join(PathBuf::from("css"));
    let (watcher, rx) = watch_directory(&repo_dir)?;

    let metadata_state = Arc::new(RwLock::new(None));
    let metadata_state_clone = Arc::clone(&metadata_state);

    let _watcher_arc = Arc::new(watcher);

    let repo_path_arc = Arc::new(repo_dir.clone());
    tokio::spawn(async move {
        let mut rx = rx;
        println!("{:?}", repo_path_arc);
        while let Some(res) = rx.recv().await {
            match res {
                Ok(_) => {
                    match build_metadata(Arc::clone(&repo_path_arc)) {
                        Ok(metadata) => {
                            println!("[+] metadata built successfully");

                            // Update the shared state
                            let mut metadata_lock = metadata_state_clone.write().unwrap();
                            *metadata_lock = Some(metadata);
                        }
                        Err(e) => println!("[-] error building metadata: {}", e),
                    }
                }
                Err(e) => println!("Watch error: {:?}", e),
            }
        }
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/get_metadata_all", get(get_metadata_all))
        .route("/get_package_metadata", get(get_package_metadata))
        .nest_service("/repo", ServeDir::new(&repo_dir))
        .nest_service("/css", ServeDir::new(&static_dir))
        .layer(Extension(metadata_state))
        .layer(Extension(Arc::new(repo_dir)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    serve(listener, app).await.unwrap();

    Ok(())
}
