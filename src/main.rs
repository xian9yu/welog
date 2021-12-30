use axum::{
    handler::Handler,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, ORIGIN},
        Method,
    },
    routing::{get, post},
    AddExtensionLayer, Router,
};
use tokio::signal;
use tower::layer::layer_fn;
use tower_http::cors::{CorsLayer, Origin};

use crate::handler::index::handler_404;

mod config;
mod db;
mod handler;
mod middleware;
mod models;
mod router;

pub(crate) fn service_router() -> Router {
    Router::new()
        .route("/", get(handler::index::index))
        .route("/register", post(handler::user::register))
        .nest("/user", router::user())
        .nest("/article", router::article())
}

#[tokio::main]
async fn main() {
    // route
    let router = service_router()
        // add a fallback service for handling routes to unknown paths
        .fallback(handler_404.into_service())
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

    // run it with hyper on 0.0.0.0:1234
    let addr = config::get_env("IP_ADDRESS");
    println!("listening on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// 优雅关机
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
