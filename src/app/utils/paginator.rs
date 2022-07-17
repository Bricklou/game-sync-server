use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct PaginatorQuery {
    #[validate(range(min = 1))]
    pub page: Option<i64>,
    #[validate(range(min = 1))]
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginatorObject {
    pub page: i64,
    pub per_page: i64,
    pub search: Option<String>,
}

impl PaginatorQuery {
    pub fn into_object(self) -> PaginatorObject {
        PaginatorObject {
            page: self.page.unwrap_or(1),
            per_page: self.per_page.unwrap_or(10),
            search: self.search,
        }
    }
}

impl PaginatorObject {
    pub fn new(page: i64, per_page: i64, search: Option<String>) -> Self {
        Self {
            page,
            per_page,
            search,
        }
    }

    pub fn default() -> Self {
        Self::new(1, 10, None)
    }
}

#[derive(Serialize)]
pub struct PaginatorResponse<T> {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub items: Vec<T>,
}
