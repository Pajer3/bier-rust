use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject};
use crate::definitions::user::User;
use crate::utils::{crypto, fs_util, auth};
use sqlx::types::Uuid;
use sqlx::Row;

#[derive(SimpleObject)]
pub struct RegisterResponse {
    pub user: User,
    pub token: String,
    pub encrypted_metadata: String,
}

#[derive(SimpleObject)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
    pub encrypted_metadata: String,
}

pub type BierSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "Hello World"
    }

    async fn me(&self, ctx: &Context<'_>) -> Result<User, async_graphql::Error> {
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;

        let user_record = sqlx::query!(
            "SELECT id, email, password_hash, display_name, avatar_url, is_verified, created_at FROM users WHERE id = $1",
            auth_user.id
        )
        .fetch_optional(pool)
        .await?;

        match user_record {
            Some(u) => Ok(User {
                id: u.id,
                display_name: u.display_name,
                email: u.email,
                password_hash: u.password_hash,
                avatar_url: u.avatar_url.unwrap_or_default(),
                is_verified: u.is_verified.unwrap_or(false),
                created_at: u.created_at.expect("Timestamp missing"),
            }),
            None => Err("Gebruiker niet gevonden".into()),
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn logout_user(&self, ctx: &Context<'_>) -> Result<bool, async_graphql::Error> {
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;

        sqlx::query!(
            "DELETE FROM sessions WHERE id = $1",
            auth_user.session_id
        )
        .execute(pool)
        .await?;

        Ok(true)
    }

    async fn login_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> Result<LoginResponse, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing in context")?;

        let user_record = sqlx::query!(
            "SELECT id, email, password_hash, display_name, avatar_url, is_verified, created_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pool)
        .await?;

        let user_record = match user_record {
            Some(u) => u,
            None => return Err("E-mailadres of wachtwoord onjuist".into()),
        };

        if !crypto::verify_password(&password, &user_record.password_hash) {
            return Err("E-mailadres of wachtwoord onjuist".into());
        }
        let encrypted_metadata_hex = fs_util::load_encrypted_metadata(user_record.id).await?;

        let meta = ctx.data::<crate::RequestMetadata>().ok();
        let user_agent = meta.and_then(|m| m.user_agent.clone());
        

        let expires_at = time::OffsetDateTime::now_utc() + time::Duration::days(365);
        
        let mut tx = pool.begin().await.map_err(|_| "Database transaction failed")?;

        let session_record = sqlx::query!(
            "INSERT INTO sessions (user_id, user_agent, expires_at) VALUES ($1, $2, $3) RETURNING id",
            user_record.id,
            user_agent,
            expires_at
        )
        .fetch_one(&mut *tx)
        .await?;

        // Voorkom zombie sessions door token te maken voor commit
        let token = auth::create_jwt(user_record.id, &session_record.id.to_string())?;

        tx.commit().await.map_err(|_| "Database commit failed")?;


        Ok(LoginResponse {
            user: User {
                id: user_record.id,
                display_name: user_record.display_name,
                email: user_record.email,
                password_hash: user_record.password_hash,
                avatar_url: user_record.avatar_url.unwrap_or_default(),
                is_verified: user_record.is_verified.unwrap_or(false),
                created_at: user_record.created_at.expect("Timestamp missing"),
            },
            token,
            encrypted_metadata: encrypted_metadata_hex,
        })
    }

    async fn register_user(
        &self,
        ctx: &Context<'_>,
        username: String,
        email: String,
        password: String,
    ) -> Result<RegisterResponse, async_graphql::Error> {
        if username.trim().len() < 3 {
             return Err("Gebruikersnaam moet minimaal 3 tekens zijn".into());
        }
        if !email.contains('@') || email.len() < 5 {
            return Err("Ongeldig e-mailadres".into());
        }
        if password.len() < 8 {
            return Err("Wachtwoord moet minimaal 8 tekens zijn".into());
        }

        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing in context")?;
        
        let mut tx = pool.begin().await?;
        
        let existing = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
            .fetch_optional(&mut *tx)
            .await?;

        if existing.is_some() {
            return Err("Er bestaat al een account met dit e-mailadres".into());
        }

        let password_hash = crypto::hash_password(&password)?;
        let default_avatar_base64 = fs_util::load_default_avatar().await;

        let record = sqlx::query!(
            "INSERT INTO users (email, password_hash, display_name, avatar_url) VALUES ($1, $2, $3, $4) RETURNING id, created_at",
            email,
            password_hash,
            username,
            default_avatar_base64
        )
        .fetch_one(&mut *tx)
        .await?;

        let user_id = record.id;
        let created_at = record.created_at.expect("Timestamp missing");

        let user_dir = fs_util::create_user_directory(user_id).await?;
        
        let metadata = crate::definitions::metadata::Metadata {
            id: user_id,
            email: email.clone(),
            latest_session: crate::definitions::metadata::Session {
                email: email.clone(),
                created_at: created_at.to_string(),
            },
        };
        
        let encrypted_metadata_hex = crypto::encrypt_metadata(&metadata)?;
        
        fs_util::save_encrypted_metadata(&user_dir, &encrypted_metadata_hex).await?;
        
        let token_row = sqlx::query(
            "INSERT INTO verification_tokens (user_id, token_type, expires_at) VALUES ($1, $2, $3) RETURNING id"
        )
        .bind(user_id)
        .bind("VERIFY")
        .bind(time::OffsetDateTime::now_utc() + time::Duration::hours(24))
        .fetch_one(&mut *tx)
        .await?;

        let token_id: Uuid = token_row.get("id");

        let meta = ctx.data::<crate::RequestMetadata>().ok();
        let user_agent = meta.and_then(|m| m.user_agent.clone());
        let expires_at = time::OffsetDateTime::now_utc() + time::Duration::days(365);
        
        let session_record = sqlx::query!(
            "INSERT INTO sessions (user_id, user_agent, expires_at) VALUES ($1, $2, $3) RETURNING id",
            user_id,
            user_agent,
            expires_at
        )
        .fetch_one(&mut *tx)
        .await?;

        // Voorkom 'zombie users' door de JWT te genereren voordat we committen
        let token = auth::create_jwt(user_id, &session_record.id.to_string())?;

        tx.commit().await?;

        // Email sturen doen we na de commit, anders wachten we onnodig lang
        let _ = crate::utils::email::send_verification_email(&email, &token_id.to_string()).await;


        Ok(RegisterResponse {
            user: User {
                id: user_id,
                display_name: username,
                email,
                password_hash,
                avatar_url: default_avatar_base64,
                is_verified: false,
                created_at,
            },
            token,
            encrypted_metadata: encrypted_metadata_hex,
        })
    }

    async fn verify_email(&self, ctx: &Context<'_>, token: String) -> Result<bool, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let token_uuid = Uuid::parse_str(&token).map_err(|_| "Ongeldig token")?;

        let mut tx = pool.begin().await?;

        let token_row = sqlx::query(
            "SELECT user_id FROM verification_tokens WHERE id = $1 AND token_type = $2 AND expires_at > NOW()"
        )
        .bind(token_uuid)
        .bind("VERIFY")
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(row) = token_row {
            let user_id: i32 = row.get("user_id");
            
            sqlx::query(
                "UPDATE users SET is_verified = TRUE WHERE id = $1"
            )
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                "DELETE FROM verification_tokens WHERE id = $1"
            )
            .bind(token_uuid)
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
            Ok(true)
        } else {
            Err("Token ongeldig of verlopen".into())
        }
    }

    async fn forgot_password(&self, ctx: &Context<'_>, email: String) -> Result<bool, async_graphql::Error> {
        if !email.contains('@') || email.len() < 5 {
            return Err("Ongeldig e-mailadres".into());
        }
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;

        let user = sqlx::query!("SELECT id FROM users WHERE email = $1", email)
            .fetch_optional(pool)
            .await?;

        if let Some(u) = user {
            let token_row = sqlx::query(
                "INSERT INTO verification_tokens (user_id, token_type, expires_at) VALUES ($1, $2, $3) RETURNING id"
            )
            .bind(u.id)
            .bind("RESET")
            .bind(time::OffsetDateTime::now_utc() + time::Duration::hours(1))
            .fetch_one(pool)
            .await?;

            let token_id: Uuid = token_row.get(0);
            let _ = crate::utils::email::send_reset_email(&email, &token_id.to_string()).await;
        }

        Ok(true)
    }

    async fn reset_password(
        &self,
        ctx: &Context<'_>,
        token: String,
        new_password: String,
    ) -> Result<bool, async_graphql::Error> {
        if new_password.len() < 8 {
            return Err("Wachtwoord moet minimaal 8 tekens zijn".into());
        }

        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let token_uuid = Uuid::parse_str(&token).map_err(|_| "Ongeldig token")?;

        let mut tx = pool.begin().await?;

        let token_row = sqlx::query(
            "SELECT user_id FROM verification_tokens WHERE id = $1 AND token_type = $2 AND expires_at > NOW()"
        )
        .bind(token_uuid)
        .bind("RESET")
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(row) = token_row {
            let user_id: i32 = row.get("user_id");
            let password_hash = crypto::hash_password(&new_password)?;
            
            sqlx::query(
                "UPDATE users SET password_hash = $1 WHERE id = $2"
            )
            .bind(password_hash)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                "DELETE FROM verification_tokens WHERE user_id = $1 AND token_type = $2"
            )
            .bind(user_id)
            .bind("RESET")
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
            Ok(true)
        } else {
            Err("Token ongeldig of verlopen".into())
        }
    }
}
