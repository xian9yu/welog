use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
// 关联
pub struct Links {
    pub aid: Option<u64>, // article id
    pub mid: Option<u64>, // metas id
}

impl Default for Links {
    fn default() -> Self {
        Self {
            aid: None,
            mid: None,
        }
    }
}
