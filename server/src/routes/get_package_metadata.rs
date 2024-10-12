use crate::utils::builder::{ExtendedEntry, Metadata, PackageMetadata};
use anyhow::Result;
use axum::{extract::Query, Extension, Json};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use thiserror::Error;
use zip::ZipArchive;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(r#"Error parsing metadata"#)]
    ParseError(),
    #[error(r#"No package found"#)]
    NoPackageError()
}

fn get_question_count(zip_path: PathBuf) -> Result<u64> {
    let file = File::open(&zip_path)?;
    let mut zip = ZipArchive::new(file)?;

    let mut txt_file_count = 0;

    for i in 0..zip.len() {
        let file = zip.by_index(i)?;
        let name = file.name();

        if Path::new(name).extension().and_then(|s| s.to_str()) == Some("txt") {
            txt_file_count += 1;
        }
    }
    Ok(txt_file_count)
}

fn parse_metadata(
    metadata: &Metadata,
    hash: &str,
    repo_path: Arc<PathBuf>,
) -> Result<ExtendedEntry> {
    for entry in &metadata.entries {
        if hash == entry.hash {
            let local_path = PathBuf::from(&entry.path[1..]).parent().unwrap().to_path_buf();
            let filepath = repo_path
                .parent()
                .unwrap().join(local_path);
            if let Ok(contents) = fs::read_to_string(filepath.join(PathBuf::from("metadata.json")))
            {
                let package_metadata: PackageMetadata = serde_json::from_str(&contents)?;
                return Ok(ExtendedEntry {
                    hash: entry.hash.clone(),
                    name: entry.name.clone(),
                    version: entry.version.clone(),
                    course_id: entry.course_id.clone(),
                    path: entry.path.clone(),
                    description: package_metadata.description,
                    author: package_metadata.author,
                    question_amount: get_question_count(
                        repo_path
                            .parent()
                            .unwrap_or(Path::new("/"))
                            .join(&entry.path[1..]),
                    )?,
                    format: package_metadata.format,
                });
            } else {
                return Err(ParseError::ParseError().into());
            }
        }
    }
    Err(ParseError::NoPackageError().into())
}

pub async fn get_package_metadata(
    Query(params): Query<HashMap<String, String>>,
    Extension(metadata_state): Extension<Arc<RwLock<Option<Metadata>>>>,
    Extension(repo_path): Extension<Arc<PathBuf>>,
) -> Json<Value> {
    if let Some(hash) = params.get("hash") {
        let metadata_lock = metadata_state.read().unwrap();
        if let Some(metadata) = &*metadata_lock {
            match parse_metadata(metadata, hash, repo_path) {
                Ok(extended) => return Json(json!(extended)),
                Err(e) => return Json(json!({"error": format!("{:?}", e)})) 
            } 
        } else {
            return Json(json!({"error": "Metadata not available"}));
        }
    } else {
        return Json(json!({"error": "No package hash provided"}));
    }
}
