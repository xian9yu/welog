use dotenv::dotenv;
use std::env;

// 获取 .env 文件里 key 的值 
pub(crate) fn get_env(key: &str) -> String {
    dotenv().ok();
    let msg = format!("{}{}", ".env file must be set field: ", key);
    // let msg = .to_string() + key;
    env::var(key).expect(&msg)
}
