pub mod index;
pub mod user;

// pub modules

// 分页
#[derive(serde::Deserialize)]
pub(crate) struct Pagination {
    pub page_no: usize,
    pub page_size: usize,
}
