use std::{fs, path::PathBuf, sync::Arc};
use serde::{self, Deserialize, Serialize};
use anyhow::Result;
use thiserror::Error;
use md5;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(r#"Error reading metadata ({0}): {1}"#)]
    ReadError(PathBuf, String)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    entries: Vec<ShortEntry>
}

#[derive(Serialize, Deserialize, Debug)]
struct ExtendedEntry {
    md5: String,
    name: String,
    version: String,
    course_id: String,
    description: String,
    author: String,
    question_amount: u64,
    date: String,
    format: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortEntry {
    md5: String,
    name: String,
    version: String,
    path: String,
    course_id: String
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageMetadata {
    name: String,
    version: String,
    course_id: String,
    description: String

}

fn to_short_entry(package_metadata: PackageMetadata, path: PathBuf) -> ShortEntry {
    
}


fn parse_package_metadata(package_path: PathBuf) -> Result<PackageMetadata> {
    let metadata_path = package_path.join(PathBuf::from("metadata.json"));
    let package_metadata: PackageMetadata = match fs::read_to_string(package_path) {
        Ok(contents) => serde_json::from_str(&contents)?,
        Err(e) => {return Err(ParseError::ReadError(metadata_path, e.to_string()).into())}
    };
    Ok(package_metadata)
}

pub fn build_metadata(metadata_path: Arc<PathBuf>) -> Result<Metadata> {
    let mut metadata: Vec<Metadata> = Vec::new();
    fs::read_dir(metadata_path.as_path())?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .for_each(|entry| {
            let path = entry.path();
            let metadata_path = path.join("metadata.json");

            if metadata_path.is_file() {
                if let Ok(pakage_metadata) = parse_package_metadata(package_path) {
                    metadata.append();
                }
            } else {
                println!("'metadata.json' not found in {}", path.display());
            }
        });
}