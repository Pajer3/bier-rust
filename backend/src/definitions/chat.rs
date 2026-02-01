use async_graphql::{SimpleObject, InputObject};
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, sqlx::FromRow)]
pub struct ClubMessage {
    pub id: i32,
    pub club_id: i32,
    pub user_id: Option<i32>, // Nullable if user deleted?
    pub content: String, // Encrypted in DB, Decrypted in struct/API? Or decrypted by resolver? 
                         // We will store encrypted content here and decrypt in resolver field.
                         // But FromRow will pull usage directly. 
                         // Wait, if I want to decrypt, I should probably have a separate field or logic.
                         // For simplicity, let's say the struct holds the DECRYPTED content if possible, or we handle it in resolver.
                         // Actually, sqlx::FromRow maps DB columns. DB column is 'content' (encrypted text).
                         // We should maybe rename struct field to 'encrypted_content' or handle decryption manually.
                         // Let's use 'content' matching DB, but note it might be encrypted. 
                         // Ideally we use a resolver method to expose 'decrypted_content'.
    #[graphql(skip)]
    pub created_at: Option<OffsetDateTime>,
    
    // Extra fields for UI optimization if we join user
    #[sqlx(default)] 
    pub user_display_name: Option<String>,
    #[sqlx(default)]
    pub user_avatar_url: Option<String>,
}

#[derive(InputObject)]
pub struct SendMessageInput {
    pub club_id: i32,
    pub content: String, // Plaintext from client
}
