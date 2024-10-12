use std::{fs, path::PathBuf, sync::Arc};
use serde::{self, Deserialize, Serialize};
use anyhow::Result;
use thiserror::Error;

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

fn parse_package_metadata(package_path: PathBuf) -> Result<PackageMetadata> {
    let metadata_path = package_path.join(PathBuf::from("metadata.json"));
    let package_metadata: PackageMetadata = match fs::read_to_string(package_path) {
        Ok(contents) => serde_json::from_str(&contents)?,
        Err(e) => {return Err(ParseError::ReadError(metadata_path, e.to_string()).into())}
    };
    todo!()
}

pub fn build_metadata(metadata_path: Arc<PathBuf>) -> Result<Metadata> {

    todo!()
}