use crate::config;
use rbatis::rbatis::Rbatis;
use std::sync::Arc;

pub(crate) async fn init_sql() -> Arc<Rbatis> {
    log::info!("Connecting to database...");

    // get db url
    let database_url = config::get_env("DATABASE_URL");

    let rb = Rbatis::new();
    rb.link(&database_url)
        .await
        .expect("rbatis database connection failed");

    Arc::new(rb)
}
