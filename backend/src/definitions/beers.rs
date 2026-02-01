use async_graphql::{SimpleObject, InputObject};
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Beer {
    pub id: Uuid,
    pub name: String,
    pub brewery: Option<String>,
    pub r#type: Option<String>, // "type" is a reserved keyword
    pub abv: Option<f64>,       // SQL float corresponds to f64 usually (or f32)
    pub ibu: Option<i32>,
    pub color: Option<String>,
    pub image_url: Option<String>,
    pub created_by: Option<i32>,
    #[graphql(skip)] // GraphQL doesn't always handle OffsetDateTime automatically without scalar config, but usually async-graphql "time" feature handles it. Use scalar or skip if complex. But let's try exposing it.
    pub created_at: Option<OffsetDateTime>,
}

#[derive(InputObject)]
pub struct CreateBeerInput {
    pub name: String,
    pub brewery: Option<String>,
    pub r#type: Option<String>,
    pub abv: Option<f64>,
    pub ibu: Option<i32>,
    pub color: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct Review {
    pub id: Uuid,
    pub user_id: i32,
    pub beer_id: Uuid,
    pub rating: i32,
    pub text: Option<String>,
    #[graphql(skip)]
    pub created_at: Option<OffsetDateTime>,
}

#[derive(InputObject)]
pub struct CreateReviewInput {
    pub beer_id: Uuid,
    pub rating: i32,
    pub text: Option<String>,
}

#[derive(InputObject)]
pub struct BeerFilter {
    pub search: Option<String>,  // Matches name or brewery
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ReviewWithUser {
    pub id: Uuid,
    pub user_id: i32,
    pub user_name: String,
    pub beer_id: Uuid,
    pub rating: i32,
    pub text: Option<String>,
    #[graphql(skip)]
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct BeerDetail {
    pub beer: Beer,
    pub average_rating: f64,
    pub review_count: i64,
    pub reviews: Vec<ReviewWithUser>,
}
