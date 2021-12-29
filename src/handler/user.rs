use std::sync::Arc;

use crate::models::mod_user::{self, User};
use axum::{
    extract::{self, Extension, Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
};
use chrono::Local;
use rbatis::rbatis::Rbatis;
use serde_json::json;

/**
 * TODO (dm5, salt, 邮箱验证)
 */
// user register
pub(crate) async fn register(
    rb: Extension<Arc<Rbatis>>,
    extract::Form(form): extract::Form<mod_user::User>,
) -> impl IntoResponse {
    // Convert data type
    let mail = form.mail.unwrap_or_default();
    let password = form.password.unwrap_or_default();

    // check mail lenth
    if mail.clone().len() < 6 {
        return (StatusCode::OK, Json(json!({"error":"邮箱不合法"})));
    }

    // check password lenth
    if password.clone().len() < 6 {
        return (StatusCode::OK, Json(json!({"error":"密码太短"})));
    }

    // check mail is exist
    let mail_exist = mod_user::User::count(rb.clone(), "mail", &mail).await;
    if mail_exist > 0 {
        return (StatusCode::OK, Json(json!({"error":"当前 mail 已注册"})));
    }

    let user = mod_user::User {
        uid: None,
        nickname: None,
        password: Some(password),
        mail: Some(mail),
        url: None,
        last_login_time: None,
        create_time: Some(Local::now().timestamp()),
    };
    // sql
    let v = mod_user::User::add(rb, user).await.unwrap_or(1);

    (
        StatusCode::OK,
        Json(json!({ "data": format!("{}{}", "注册成功,id: ", v) })),
    )
}

// delete user
pub(crate) async fn delete(
    rb: Extension<Arc<Rbatis>>,
    Path(user): Path<mod_user::User>,
) -> impl IntoResponse {
    if let Some(uid) = user.uid {
        // check uid existence
        let uid_exist = mod_user::User::count(rb.clone(), "uid", &uid).await;
        if uid_exist < 1 {
            let error_message = format!("{}{}{}", "删除失败,用户 id=<", uid, "> 不存在");
            return (StatusCode::OK, Json(json!({ "error": error_message })));
        }
        // sql
        let res = mod_user::User::delete(rb, "uid", uid).await;
        if res < 1 {
            let error_message = format!("{}{}{}", "删除用户 id=<", uid, "> 失败");
            return (StatusCode::OK, Json(json!({ "error": error_message })));
        }

        let message = format!("{}{}{}", "删除用户 id=<", uid, "> 成功");
        return (StatusCode::OK, Json(json!({ "data": message })));
    }

    (StatusCode::OK, Json(json!({ "error": "意想不到的错误" })))
}

// edit
pub(crate) async fn edit() -> Html<&'static str> {
    mod_user::User::edit().await
}

/*
// query
*/
// get users info
pub(crate) async fn get_user_info(
    rb: Extension<Arc<Rbatis>>,
    Path(user): Path<mod_user::User>,
) -> impl IntoResponse {
    let mut v: Vec<User> = vec![];

    // ... by nickname
    if let Some(nickname) = user.nickname {
        // check uid is exist
        let nickname_count = mod_user::User::count(rb.clone(), "nickname", &nickname).await;
        if nickname_count < 1 {
            return (
                StatusCode::OK,
                Json(json!({"error":"当前 nickname 不存在"})),
            );
        }
        v = mod_user::User::get_user_by(rb, "nickname", &nickname).await;

    // ... by mail
    } else if let Some(mail) = user.mail {
        // check mail is exist
        let mail_count = mod_user::User::count(rb.clone(), "mail", &mail).await;
        if mail_count < 1 {
            return (StatusCode::OK, Json(json!({"error":"当前 mail 不存在"})));
        }
        v = mod_user::User::get_user_by(rb, "mail", &mail).await;

    // ... by uid
    } else if let Some(uid) = user.uid {
        // check uid is exist
        let uid_count = mod_user::User::count(rb.clone(), "uid", &uid).await;
        if uid_count < 1 {
            return (StatusCode::OK, Json(json!({"error":"当前 uid 不存在"})));
        }
        v = mod_user::User::get_user_by(rb, "uid", &uid).await;
    }

    (StatusCode::OK, Json(json!({ "data": v })))
}

// get users list
pub(crate) async fn get_user_list(
    rb: Extension<Arc<Rbatis>>,
    pagination: Query<super::Pagination>,
) -> impl IntoResponse {
    let page_no = pagination.page_no.try_into().unwrap();
    let page_size = pagination.page_size.try_into().unwrap();
    // sql
    let v = mod_user::User::get_list(rb, page_no, page_size).await;
    Json(v)
}