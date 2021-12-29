use axum::{
    routing::{get, post},
    Router,
};

use crate::handler;

pub(crate) fn user() -> Router {
    Router::new()
        .route("/delete/uid/:uid", get(handler::user::delete))
        .route("/edit", post(handler::user::edit))
        .route("/id/:uid", get(handler::user::get_user_info))
        .route("/mail/:mail", get(handler::user::get_user_info))
        .route("/nickname/:nickname", get(handler::user::get_user_info))
        .route("/list", get(handler::user::get_user_list))
}

pub(crate) fn article() -> Router {
    Router::new()
}
