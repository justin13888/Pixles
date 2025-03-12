use async_graphql::*;

pub struct UserStatisticsQuery;

#[Object]
impl UserStatisticsQuery {
    /// Total photos
    async fn total_photos(&self) -> i64 {
        11
    }

    /// Total albums
    async fn total_albums(&self) -> i64 {
        12
    }

    /// Storage used in bytes
    async fn storage_used(&self) -> i64 {
        1234567890
    }
}
