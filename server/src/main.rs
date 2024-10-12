mod utils;

use anyhow::Result;
use axum::{serve, Extension, Router};
use std::{
    env, fs::File, path::PathBuf, sync::{Arc, RwLock}
};
use tower_http::services::ServeDir;

use utils::{watcher::watch_directory, builder::build_metadata};

#[tokio::main]
async fn main() -> Result<()> {
    let repo_dir= PathBuf::from(env::var("REPO_DIR").unwrap_or(String::from("./repo")));
    let metadata_dir= PathBuf::from(env::var("METADATA_DIR").unwrap_or(String::from("./metadata")));
    let metadata_path  = metadata_dir.join(PathBuf::from("metadata.json"));
    let rx = watch_directory(&repo_dir)?;

    let metadata_state = Arc::new(RwLock::new(None));
    let metadata_state_clone = Arc::clone(&metadata_state);
    
    match File::create_new(&metadata_path) {
        Ok(_) => {println!("[+] metadata.json created")},
        Err(_) => {println!("[+] metadata.json present on disk")}
    }

    let metadata_path_arc = Arc::new(metadata_path.clone());
    tokio::spawn(async move {
        let mut rx = rx;
        while let Some(res) = rx.recv().await {
            match res {
                Ok(_) => {
                    match build_metadata(Arc::clone(&metadata_path_arc)) {
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
        .nest_service("/repo", ServeDir::new(repo_dir))
        .layer(Extension(metadata_state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    serve(listener, app).await.unwrap();

    Ok(())
}
