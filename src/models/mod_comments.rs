use chrono::Local;
use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Comments {
    pub cid: Option<u64>,              // 评论 id
    pub aid: Option<String>,           // article id
    pub content: Option<String>,       // 评论内容
    pub author_id: Option<u64>,        // 文章所有者
    pub visitor_name: Option<String>,  // 访客评论
    pub visitor_mail: Option<String>,  // 邮箱
    pub visitor_url: Option<String>,   // web
    pub visitor_ip: Option<String>,    // ip
    pub visitor_agent: Option<String>, // agent
    pub types: Option<String>,         // 类型
    pub status: Option<String>,        // 状态
    pub parent: Option<String>,        // 父类 id
    pub create_time: Option<i64>,
}

impl Default for Comments {
    fn default() -> Self {
        Self {
            cid: None,
            aid: None,
            content: None,
            author_id: None,
            visitor_name: None,
            visitor_mail: None,
            visitor_url: None,
            visitor_ip: None,
            visitor_agent: None,
            types: None,
            status: None,
            parent: None,
            create_time: Some(Local::now().timestamp()),
        }
    }
}
