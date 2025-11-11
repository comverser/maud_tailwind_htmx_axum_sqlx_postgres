use serde::Deserialize;

/// Default page number for pagination queries
pub fn default_page() -> i64 {
    1
}

/// Base query struct for paginated endpoints
#[derive(Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: i64,
}
