use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;
use serde::{self, Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
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
struct ShortEntry {
    md5: String,
    name: String,
    version: String,
    path: String,
    course_id: String
}

struct PackageMetadata {
    name: String,
    version: String,
    course_id: String,
    description: String
}

pub fn build_metadata(metadata_path: Arc<PathBuf>) -> Result<()> {
     todo!()
}