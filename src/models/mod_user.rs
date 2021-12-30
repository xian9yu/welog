use std::{fmt::Debug, sync::Arc};

use axum::{extract::Extension, response::Html};
use chrono::Local;
use rbatis::{crud::CRUD, crud_table, rbatis::Rbatis, Page, PageRequest};

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub uid: Option<u64>,             // 用户id
    pub nickname: Option<String>,     // 显示昵称
    pub password: Option<String>,     // 密码
    pub mail: Option<String>,         // 邮箱
    pub url: Option<String>,          // 个人网站url
    pub last_login_time: Option<i64>, // 最近登录时间
    pub create_time: Option<i64>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            uid: None,
            nickname: None,
            password: None,
            mail: None,
            url: None,
            last_login_time: Some(Local::now().timestamp()),
            create_time: Some(Local::now().timestamp()),
        }
    }
}

impl User {
    // add
    pub(crate) async fn add(rb: Extension<Arc<Rbatis>>, user: User) -> Option<i64> {
        rb.save(&user, &[]).await.unwrap().last_insert_id
    }

    // edit
    pub(crate) async fn edit() -> Html<&'static str> {
        Html("<h2>你好, World!</h2>")
    }

    // delete
    pub(crate) async fn delete<T>(rb: Extension<Arc<Rbatis>>, column: &str, val: T) -> u64
    where
        T: serde::Serialize + Sync + Send + Copy + Debug,
    {
        rb.remove_by_column::<User, _>(column, val).await.unwrap()
    }

    // get user info
    pub(crate) async fn get_user_by<T>(
        rb: Extension<Arc<Rbatis>>,
        column: &str,
        val: T,
    ) -> Vec<User>
    where
        T: serde::Serialize + Sync + Send + Copy + Debug,
    {
        rb.fetch_list_by_column(column, &[val]).await.unwrap()
    }

    // get count
    pub(crate) async fn count<T>(rb: Extension<Arc<Rbatis>>, column: &str, val: T) -> u64
    where
        T: serde::Serialize + Copy + Debug,
    {
        let count = rb.new_wrapper().eq(&column, val);
        rb.fetch_count_by_wrapper::<User>(count).await.unwrap()
    }

    // get user list
    pub(crate) async fn get_list(
        rb: Extension<Arc<Rbatis>>,
        page_no: u64,
        page_size: u64,
    ) -> Page<User> {
        let wraper = rb.new_wrapper();
        // sql
        let data: Page<User> = rb
            .fetch_page_by_wrapper(wraper, &PageRequest::new(page_no, page_size))
            .await
            .unwrap();
        data
    }
}
