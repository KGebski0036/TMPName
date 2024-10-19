pub mod builder;
pub mod watcher;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SplitError {
    #[error(r#"No /repo in path"#)]
    NoRepo(),
}

pub fn split_at_repo(path: String) -> Result<String> {
    if let Some(index) = path.find("/repo") {
        return Ok(path[index..].to_string());
    } else {
        return Err(SplitError::NoRepo().into());
    }
}
