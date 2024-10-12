mod utils;
mod routes;
use anyhow::Result;
use axum::{serve,routing::{get}, Router};
use std::{
    env,
    path::{Path, PathBuf},
};
use tower_http::services::ServeDir;
use routes::index;
use utils::watcher::watch_directory;

#[tokio::main]
async fn main() -> Result<()> {
    let repo_dir: PathBuf = PathBuf::from(env::var("REPO_DIR").unwrap_or(String::from("./repo")));


    let rx = watch_directory(&repo_dir)?;

    // Spawn a task to handle filesystem events
    tokio::spawn(async move {
        let mut rx = rx;
        while let Some(res) = rx.recv().await {
            match res {
                Ok(event) => {
                    println!("Filesystem event: {:?}", event);
                }
                Err(e) => println!("Watch error: {:?}", e),
            }
        }
    });

    let app = Router::new().nest_service("/static", ServeDir::new(repo_dir)).
    route("/index", get(routes::index()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    serve(listener, app).await.unwrap();

    Ok(())
}
