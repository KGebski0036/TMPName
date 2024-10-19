use axum::response::Html;
use markdown;

use std::{env, fs, path::PathBuf};
pub async fn index() -> Html<String> {
    let dir_path =
        PathBuf::from(env::var("TEMPLATE_DIR").unwrap_or("/etc/server/templates".to_string()));
    let file_content: String = fs::read_to_string(dir_path.join(PathBuf::from("index.md")))
        .expect("Unable to read index.md file");
    // Html(markdown::to_html(&file_content))
    let out_html =
        markdown::to_html_with_options(&file_content, &markdown::Options::gfm()).unwrap();
    // println!("{:?}", out_html);
    let html_extra = r#"
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
    body {
     background-color: #0d1117;
     display: grid;
    place-items: center;
    }
      .markdown-body {
        box-sizing: border-box;
        min-width: 200px;
        max-width: 980px;
        margin: 0 auto;
        padding: 45px;
      }

      @media (max-width: 767px) {
        .markdown-body {
          padding: 15px;
        }
      }
    </style>"#;
    let html_extra_formated = format!(
        "{style}<link rel=\"stylesheet\" type=\"text/css\" href=\"{path}\">",
        style = html_extra,
        path = "/css/github-markdown-dark.css" // dir_path
                                               //     .join(PathBuf::from("github-markdown.css"))
                                               //     .into_os_string()
                                               //     .into_string()
                                               //     .unwrap()
    );
    let html_final = format!(
        "<!DOCTYPE html>{style}<div class=\"markdown-body\">{html}</div>",
        style = html_extra_formated,
        html = out_html
    );
    // println!("{:?}", html_final);
    Html(html_final)
}
