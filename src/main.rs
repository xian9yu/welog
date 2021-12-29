use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, ORIGIN},
        Method,
    },
    routing::{get, post},
    AddExtensionLayer, Router,
};
use tower::layer::layer_fn;
use tower_http::cors::{CorsLayer, Origin};

use crate::middleware::auth::AuthMiddleware;

mod config;
mod db;
mod handler;
mod middleware;
mod models;
mod router;

pub fn service_router() -> Router {
    Router::new()
        .route("/", get(handler::index::index))
        .route("/register", post(handler::user::register))
        .nest("/user", router::user())
        .nest(
            "/article",
            router::article().layer(layer_fn(|inner| AuthMiddleware { inner })),
        )
}

#[tokio::main]
async fn main() {
    // route
    let router = service_router()
        // sql
        .layer(AddExtensionLayer::new(db::init_sql().await))
        // cors
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::exact("http://0.0.0.0:3000".parse().unwrap()))
                .allow_headers(vec![
                    ACCEPT,
                    AUTHORIZATION,
                    CONTENT_TYPE,
                    CONTENT_LENGTH,
                    ORIGIN,
                ])
                .allow_methods(vec![
                    Method::GET,
                    Method::PUT,
                    Method::POST,
                    Method::DELETE,
                    Method::OPTIONS,
                    Method::PATCH,
                ])
                .allow_credentials(true),
        );

    // add a fallback service for handling routes to unknown paths
    // let router = router.fallback(handler::index::handler_404.into_service());

    // run it with hyper on localhost:3000
    let addr = config::get_env("IP_ADDRESS");
    println!("listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
