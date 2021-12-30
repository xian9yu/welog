use axum::{http::StatusCode, response::{Html, IntoResponse}};

pub(crate) async fn index() -> Html<&'static str> {
    Html("<h1>你好, World!</h1>")
}

pub(crate) async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}