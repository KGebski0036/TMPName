use axum::response::Html;
use markdown;
use std::{env,fs};
pub async fn index() -> Html<String>{
    // read form env!
    let file_content: String = fs::read_to_string(env::var("TEMPLATE_DIR").unwrap())
        .expect("Unable to read index.md file");
    Html(markdown::to_html(&file_content))
} 
