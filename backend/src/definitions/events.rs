use async_graphql::{SimpleObject, InputObject, Enum};
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "rsvp_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RsvpStatus {
    Going,
    Interested,
    NotGoing,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Event {
    pub id: i32,
    pub club_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    // GraphQL needs scalar for OffsetDateTime usually, or we impl Scalar. 
    // For now we might skip or format as String if trouble. But async-graphql 'time' feature is on.
    pub starts_at: OffsetDateTime,
    pub ends_at: Option<OffsetDateTime>,
    pub created_by: Option<i32>,
    #[graphql(skip)]
    pub created_at: Option<OffsetDateTime>,
}

#[derive(InputObject)]
pub struct CreateEventInput {
    pub club_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub starts_at: OffsetDateTime,
    pub ends_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct EventAttendee {
    pub event_id: i32,
    pub user_id: i32,
    pub status: RsvpStatus,
}
