use axum::{
    Router,
    body::Body,
    routing::{any_service, get_service},
    extract::Request,
    http::StatusCode,
    error_handling::HandleErrorLayer,
};
use http::Response;
async fn index() {
    let body = Body::from("Hi from `{} /foo`");
    let res = Response::new(body);
} 
async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
