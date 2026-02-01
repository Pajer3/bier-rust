use async_graphql::{SimpleObject, InputObject, Enum};
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Owner,
    Mod,
    Member,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, sqlx::Type)]
#[sqlx(type_name = "member_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MemberStatus {
    Active,
    Banned,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Club {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub owner_id: i32,
    pub image_url: Option<String>,
    #[graphql(skip)]
    pub image_path: Option<String>,
    #[graphql(skip)]
    pub created_at: Option<OffsetDateTime>,
}

#[derive(InputObject)]
pub struct CreateClubInput {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>, // Client can upload image separately or provide URL
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ClubMember {
    pub club_id: i32,
    pub user_id: i32,
    pub role: UserRole,
    pub status: MemberStatus,
    #[graphql(skip)]
    pub joined_at: Option<OffsetDateTime>,
}

// Voor detail view, met user info
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ClubMemberWithUser {
    pub club_id: i32,
    pub user_id: i32,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub status: MemberStatus,
}
