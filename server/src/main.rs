mod utils;
mod routes;
use anyhow::Result;
use axum::{routing::get, serve, Extension, Router};
use std::{
    env, fs::File, path::PathBuf, sync::{Arc, RwLock}
};
use tower_http::services::ServeDir;

use utils::{watcher::watch_directory, builder::build_metadata};
use routes::{get_metadata_all::get_metadata_all, // index::index
};

#[tokio::main]
async fn main() -> Result<()> {
    let repo_dir= PathBuf::from(env::var("REPO_DIR").unwrap_or(String::from("./repo")));
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
        //.route("/", get(index))
        .route("/get_metadata_all", get(get_metadata_all))
        .nest_service("/repo", ServeDir::new(repo_dir))
        .layer(Extension(metadata_state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    serve(listener, app).await.unwrap();

    Ok(())
}