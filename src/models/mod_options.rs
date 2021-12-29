use rbatis::crud_table;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Options {
    pub name: Option<String>,  // 名称
    pub value: Option<String>, // 设定值
}

impl Default for Options {
    fn default() -> Self {
        Self {
            name: None,
            value: None,
        }
    }
}
