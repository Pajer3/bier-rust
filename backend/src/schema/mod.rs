use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject};
use crate::definitions::user::User;
use crate::utils::{crypto, fs_util, auth};

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
            "SELECT id, email, password_hash, display_name, avatar_url, created_at FROM users WHERE id = $1",
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
            "SELECT id, email, password_hash, display_name, avatar_url, created_at FROM users WHERE email = $1",
            email
        )
        .fetch_optional(pool)
        .await?;

        let user_record = match user_record {
            Some(u) => u,
            None => return Err("Invalid email or password".into()),
        };

        if !crypto::verify_password(&password, &user_record.password_hash) {
            return Err("Invalid email or password".into());
        }
        let encrypted_metadata_hex = fs_util::load_encrypted_metadata(user_record.id).await?;

        let meta = ctx.data::<crate::RequestMetadata>().ok();
        let user_agent = meta.and_then(|m| m.user_agent.clone());
        

        let expires_at = time::OffsetDateTime::now_utc() + time::Duration::days(365);
        let session_record = sqlx::query!(
            "INSERT INTO sessions (user_id, user_agent, expires_at) VALUES ($1, $2, $3) RETURNING id",
            user_record.id,
            user_agent,
            expires_at
        )
        .fetch_one(pool)
        .await?;

        let token = auth::create_jwt(user_record.id, &session_record.id.to_string())?;

        Ok(LoginResponse {
            user: User {
                id: user_record.id,
                display_name: user_record.display_name,
                email: user_record.email,
                password_hash: user_record.password_hash,
                avatar_url: user_record.avatar_url.unwrap_or_default(),
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
            return Err("User with this email already exists".into());
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

        tx.commit().await?;

        let meta = ctx.data::<crate::RequestMetadata>().ok();
        let user_agent = meta.and_then(|m| m.user_agent.clone());
        let expires_at = time::OffsetDateTime::now_utc() + time::Duration::days(365);
        
        let session_record = sqlx::query!(
            "INSERT INTO sessions (user_id, user_agent, expires_at) VALUES ($1, $2, $3) RETURNING id",
            user_id,
            user_agent,
            expires_at
        )
        .fetch_one(pool)
        .await?;

        let token = auth::create_jwt(user_id, &session_record.id.to_string())?;

        Ok(RegisterResponse {
            user: User {
                id: user_id,
                display_name: username,
                email,
                password_hash,
                avatar_url: default_avatar_base64,
                created_at,
            },
            token,
            encrypted_metadata: encrypted_metadata_hex,
        })
    }
}
