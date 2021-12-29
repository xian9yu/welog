use chrono::Local;
use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
// 分类/tag..
pub struct Metas {
    pub mid: Option<u64>,
    pub name: Option<String>,        // 名称
    pub slug: Option<String>,        // 缩略名
    pub types: Option<u64>,          // 类型
    pub discription: Option<String>, // 说明
    pub count: Option<u64>,          // 关联数量
    pub parent: Option<u64>,         // 父类 id
    pub create_time: Option<i64>,    // 创建时间
}

impl Default for Metas {
    fn default() -> Self {
        Self {
            mid: None,
            name: None,
            slug: None,
            types: None,
            discription: None,
            count: None,
            parent: None,
            create_time: Some(Local::now().timestamp()),
        }
    }
}
