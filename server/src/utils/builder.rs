use super::split_at_repo;
use anyhow::Result;
use md5::{Digest, Md5};
use serde::{self, Deserialize, Serialize};
use std::{
    fs::{self, read_dir, File},
    io::Read,
    path::PathBuf,
    sync::Arc,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(r#"Error reading metadata ({0}): {1}"#)]
    ReadError(PathBuf, String),

    #[error(r#"No zip archive in {0}"#)]
    NoZipError(PathBuf),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub entries: Vec<ShortEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtendedEntry {
    pub hash: String,
    pub name: String,
    pub version: String,
    pub course_id: String,
    pub description: String,
    pub path: String,
    pub author: String,
    pub question_amount: u64,
    pub format: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortEntry {
    pub hash: String,
    pub name: String,
    pub version: String,
    pub path: String,
    pub course_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub course_id: String,
    pub description: String,
    pub author: String,
    pub format: u64,
}

fn compute_file_hash(path: &PathBuf) -> Result<String> {
    // Open the file
    let mut file = File::open(path)?;
    let mut hasher = Md5::new();

    // Read the file in chunks to avoid loading the entire file into memory
    let mut buffer = [0u8; 4096];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    // Finalize the hash and convert it to a hex string
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn to_short_entry(package_metadata: PackageMetadata, path: PathBuf) -> Result<ShortEntry> {
    for entry in read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "zip" {
                    // Compute the hash of the ZIP file
                    let hash = compute_file_hash(&path)?;
                    return Ok(ShortEntry {
                        hash: hash,
                        name: package_metadata.name,
                        version: package_metadata.version,
                        path: split_at_repo(path.to_str().unwrap().to_string())?,
                        course_id: package_metadata.course_id,
                    });
                }
            }
        }
    }
    Err(ParseError::NoZipError(path).into())
}

fn parse_package_metadata(package_path: PathBuf) -> Result<PackageMetadata> {
    let metadata_path = package_path.join(PathBuf::from("metadata.json"));
    let package_metadata: PackageMetadata = match fs::read_to_string(&metadata_path) {
        Ok(contents) => serde_json::from_str(&contents)?,
        Err(e) => return Err(ParseError::ReadError(metadata_path, e.to_string()).into()),
    };
    Ok(package_metadata)
}

pub fn build_metadata(repo_path: Arc<PathBuf>) -> Result<Metadata> {
    let mut metadata: Vec<ShortEntry> = Vec::new();
    fs::read_dir(repo_path.as_path())?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .for_each(|entry| {
            let path = entry.path();
            let metadata_path = path.join("metadata.json");
            if metadata_path.is_file() {
                if let Ok(package_metadata) = parse_package_metadata(path.clone()) {
                    match to_short_entry(package_metadata, path) {
                        Ok(entry) => metadata.push(entry),
                        Err(e) => println!("{:?}", e),
                    }
                }
            } else {
                println!("'metadata.json' not found in {}", path.display());
            }
        });
    Ok(Metadata { entries: metadata })
}
