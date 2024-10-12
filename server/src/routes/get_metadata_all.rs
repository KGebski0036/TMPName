use crate::utils::builder::Metadata;
use axum::{extract::Extension, response::Json};
use serde_json::{json, Value};
use std::sync::{Arc, RwLock};

pub async fn get_metadata_all(
    Extension(metadata_state): Extension<Arc<RwLock<Option<Metadata>>>>,
) -> Json<Value> {
    let metadata_lock = metadata_state.read().unwrap();
    if let Some(metadata) = &*metadata_lock {
        Json(json!(metadata))
    } else {
        Json(json!({"error": "Metadata not available"}))
    }
}
