use chrono::Local;
use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Articles {
    pub aid: Option<u64>,
    pub title: Option<String>,       // 标题
    pub content: Option<String>,     // 内容
    pub author_id: Option<u64>,      // 作者 id
    pub status: Option<String>,      // 文章状态
    pub comments_num: Option<u64>,   // 评论数量
    pub comments_allow: Option<u64>, // 是否允许评论
    pub edit_time: Option<i64>,      // 修改时间
    pub create_time: Option<i64>,    // 创建时间
}

impl Default for Articles {
    fn default() -> Self {
        Self {
            aid: None,
            title: None,
            content: None,
            author_id: None,
            status: None,
            comments_num: None,
            comments_allow: None,
            edit_time: Some(Local::now().timestamp()),
            create_time: Some(Local::now().timestamp()),
        }
    }
}
