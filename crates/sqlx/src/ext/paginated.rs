use serde::{Deserialize, Serialize};
use crate::ext::Param;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub size: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct PaginatedParam<'a> {
    pub params: Option<&'a [Param]>,
    pub page: Option<u32>,
    pub size: Option<u64>,
}