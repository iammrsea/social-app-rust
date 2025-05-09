use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PaginatedQueryResult<D> {
    pub data: Vec<D>,
    pub pagination_info: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub has_next: bool,
}
