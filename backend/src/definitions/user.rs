use async_graphql::SimpleObject;
use time::OffsetDateTime;

#[derive(SimpleObject)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub display_name: String,
    pub password_hash: String,
    pub avatar_url: String,
    pub is_verified: bool,
    pub created_at: OffsetDateTime,
}