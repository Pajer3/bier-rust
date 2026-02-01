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

    async fn me(&self, ctx: &Context<'_>) -> Option<User> {
        let pool = ctx.data::<sqlx::PgPool>().ok()?;
        let auth_user = ctx.data::<crate::AuthUser>().ok()?;

        let rec = sqlx::query!(
            "SELECT id, email, password_hash, display_name, avatar_url, is_verified, created_at FROM users WHERE id = $1",
            auth_user.id
        )
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()?;

        Some(User {
            id: rec.id,
            display_name: rec.display_name,
            email: rec.email,
            password_hash: rec.password_hash,
            avatar_url: rec.avatar_url.unwrap_or_default(),
            is_verified: rec.is_verified.unwrap_or(false),
            created_at: rec.created_at.expect("Timestamp missing"),
        })
    }

    async fn beers(
        &self, 
        ctx: &Context<'_>,
        filter: Option<crate::definitions::beers::BeerFilter>,
    ) -> Result<Vec<crate::definitions::beers::Beer>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        
        // We can't easily use PgArguments dynamically with sqlx::query_as! macro style string building without a builder.
        // For simplicity, we'll use query_as without macro validation or handle optional params carefully.
        // Actually, simple string concatenation for optional constraints is dangerous for SQL injection if not careful, 
        // but parameters ($1, $2) are safe.

        // Simpele implementatie: we halen alles op of gebruiken simpele filtering met sqlx::query_as en dynamische WHERE.
        // sqlx QueryBuilder is hier het beste.
        
        let mut builder = sqlx::QueryBuilder::new("SELECT id, name, brewery, type as \"type\", abv, ibu, color, image_url, created_by, created_at FROM beers");
        
        if let Some(f) = filter {
            let mut has_where = false;
            
            if let Some(s) = f.search {
                builder.push(" WHERE (name ILIKE ");
                builder.push_bind(format!("%{}%", s));
                builder.push(" OR brewery ILIKE ");
                builder.push_bind(format!("%{}%", s));
                builder.push(")");
                has_where = true;
            }

            if let Some(t) = f.r#type {
                if has_where { builder.push(" AND "); } else { builder.push(" WHERE "); }
                builder.push("type ILIKE ");
                builder.push_bind(format!("%{}%", t));
            }
        }

        builder.push(" ORDER BY created_at DESC LIMIT 50");

        let beers = builder.build_query_as::<crate::definitions::beers::Beer>()
            .fetch_all(pool)
            .await?;

        Ok(beers)
    }

    async fn beer(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<crate::definitions::beers::BeerDetail, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;

        let beer = sqlx::query_as!(
            crate::definitions::beers::Beer,
            "SELECT id, name, brewery, type as \"type\", abv, ibu, color, image_url, created_by, created_at FROM beers WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or("Bier niet gevonden")?;

        let reviews = sqlx::query_as!(
            crate::definitions::beers::ReviewWithUser,
            "SELECT r.id, r.user_id, u.display_name as user_name, r.beer_id, r.rating, r.text, r.created_at 
             FROM reviews r
             JOIN users u ON r.user_id = u.id
             WHERE r.beer_id = $1
             ORDER BY r.created_at DESC",
            id
        )
        .fetch_all(pool)
        .await?;

        let count = reviews.len() as i64;
        let avg = if count > 0 {
            reviews.iter().map(|r| r.rating as f64).sum::<f64>() / (count as f64)
        } else {
            0.0
        };

        Ok(crate::definitions::beers::BeerDetail {
            beer,
            average_rating: avg,
            review_count: count,
            reviews,
        })
    }

    async fn my_beers(&self, ctx: &Context<'_>) -> Result<Vec<crate::definitions::beers::Beer>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        let beers = sqlx::query_as!(
            crate::definitions::beers::Beer,
            "SELECT id, name, brewery, type as \"type\", abv, ibu, color, image_url, created_by, created_at FROM beers WHERE created_by = $1 ORDER BY created_at DESC",
            auth_user.id
        )
        .fetch_all(pool)
        .await?;

        Ok(beers)
    }

    async fn my_reviews(&self, ctx: &Context<'_>) -> Result<Vec<crate::definitions::beers::Review>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        let reviews = sqlx::query_as!(
            crate::definitions::beers::Review,
            "SELECT id, user_id, beer_id, rating, text, created_at FROM reviews WHERE user_id = $1 ORDER BY created_at DESC",
            auth_user.id
        )
        .fetch_all(pool)
        .await?;

        Ok(reviews)
    }

    async fn clubs(&self, ctx: &Context<'_>) -> Result<Vec<crate::definitions::clubs::Club>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        
        // Return 50 most recently created clubs
        let clubs = sqlx::query_as!(
            crate::definitions::clubs::Club,
            "SELECT id, name, slug, description, owner_id, image_url, image_path, created_at FROM clubs ORDER BY created_at DESC LIMIT 50"
        )
        .fetch_all(pool)
        .await?;

        Ok(clubs)
    }

    async fn club(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<crate::definitions::clubs::Club, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;

        let club = sqlx::query_as!(
            crate::definitions::clubs::Club,
            "SELECT id, name, slug, description, owner_id, image_url, image_path, created_at FROM clubs WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or("Club niet gevonden")?;

        Ok(club)
    }

    async fn my_clubs(&self, ctx: &Context<'_>) -> Result<Vec<crate::definitions::clubs::Club>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        // Find clubs where user is a member
        let clubs = sqlx::query_as!(
            crate::definitions::clubs::Club,
            "SELECT c.id, c.name, c.slug, c.description, c.owner_id, c.image_url, c.image_path, c.created_at 
             FROM clubs c
             JOIN club_memberships m ON c.id = m.club_id
             WHERE m.user_id = $1
             ORDER BY m.joined_at DESC",
            auth_user.id
        )
        .fetch_all(pool)
        .await?;

        Ok(clubs)
    }

    async fn club_messages(
        &self,
        ctx: &Context<'_>,
        club_id: i32,
    ) -> Result<Vec<crate::definitions::chat::ClubMessage>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        // Check membership
        let is_member = sqlx::query!(
            "SELECT 1 as exists FROM club_memberships WHERE club_id = $1 AND user_id = $2 AND status = 'ACTIVE'::member_status",
            club_id,
            auth_user.id
        )
        .fetch_optional(pool)
        .await?;

        if is_member.is_none() {
            return Err("Je bent geen lid van deze club".into());
        }

        let messages = sqlx::query_as!(
            crate::definitions::chat::ClubMessage,
            "SELECT cm.id, cm.club_id as \"club_id!\", cm.user_id, cm.content as content, cm.created_at, u.display_name as user_display_name, u.avatar_url as user_avatar_url
             FROM club_messages cm
             LEFT JOIN users u ON cm.user_id = u.id
             WHERE cm.club_id = $1
             ORDER BY cm.created_at ASC LIMIT 100",
            club_id
        )
        .fetch_all(pool)
        .await?;

        // Decrypt messages
        let mut decrypted_messages = Vec::new();
        for mut msg in messages {
            match crypto::decrypt_string(&msg.content) {
                Ok(decrypted) => {
                    msg.content = decrypted;
                    decrypted_messages.push(msg);
                },
                Err(_) => {
                    msg.content = "⚠️ Bericht kon niet ontsleuteld worden".to_string();
                    decrypted_messages.push(msg);
                }
            }
        }

        Ok(decrypted_messages)
    }

    async fn events(&self, ctx: &Context<'_>) -> Result<Vec<crate::definitions::events::Event>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        
        let events = sqlx::query_as!(
            crate::definitions::events::Event,
            "SELECT id, club_id, title, description, location, starts_at, ends_at, created_by, created_at FROM events ORDER BY starts_at ASC LIMIT 50"
        )
        .fetch_all(pool)
        .await?;

        Ok(events)
    }

    async fn event(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<crate::definitions::events::Event, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;

        let event = sqlx::query_as!(
            crate::definitions::events::Event,
            "SELECT id, club_id, title, description, location, starts_at, ends_at, created_by, created_at FROM events WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await?
        .ok_or("Event niet gevonden")?;

        Ok(event)
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
        let email = crate::utils::sanitization::sanitize_email(&email);
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
        let email = crate::utils::sanitization::sanitize_email(&email);
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
        let email = crate::utils::sanitization::sanitize_email(&email);
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
        // ... (existing implementation)
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

    async fn create_beer(
        &self,
        ctx: &Context<'_>,
        input: crate::definitions::beers::CreateBeerInput,
    ) -> Result<crate::definitions::beers::Beer, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Je moet ingelogd zijn om een bier toe te voegen")?;

        let beer = sqlx::query_as!(
            crate::definitions::beers::Beer,
            "INSERT INTO beers (name, brewery, type, abv, ibu, color, image_url, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id, name, brewery, type as \"type\", abv, ibu, color, image_url, created_by, created_at",
            input.name,
            input.brewery,
            input.r#type,
            input.abv,
            input.ibu,
            input.color,
            input.image_url,
            auth_user.id
        )
        .fetch_one(pool)
        .await?;

        Ok(beer)
    }

    async fn rate_beer(
        &self,
        ctx: &Context<'_>,
        input: crate::definitions::beers::CreateReviewInput,
    ) -> Result<crate::definitions::beers::Review, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Je moet ingelogd zijn om een review te plaatsen")?;

        if input.rating < 1 || input.rating > 5 {
            return Err("Rating moet tussen 1 en 5 zijn".into());
        }

        let review = sqlx::query_as!(
            crate::definitions::beers::Review,
            "INSERT INTO reviews (user_id, beer_id, rating, text) VALUES ($1, $2, $3, $4) 
             ON CONFLICT (user_id, beer_id) DO UPDATE SET rating = $3, text = $4, created_at = CURRENT_TIMESTAMP
             RETURNING id, user_id, beer_id, rating, text, created_at",
            auth_user.id,
            input.beer_id,
            input.rating,
            input.text
        )
        .fetch_one(pool)
        .await?;

        Ok(review)
    }

    async fn delete_beer(
        &self,
        ctx: &Context<'_>,
        beer_id: Uuid,
    ) -> Result<bool, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        // Check ownership (only creator can delete for now, or admin if we had roles)
        let result = sqlx::query!(
            "DELETE FROM beers WHERE id = $1 AND created_by = $2",
            beer_id,
            auth_user.id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err("Bier niet gevonden of je hebt geen rechten".into());
        }

        Ok(true)
    }

    async fn update_beer(
        &self,
        ctx: &Context<'_>,
        beer_id: Uuid,
        input: crate::definitions::beers::CreateBeerInput,
    ) -> Result<crate::definitions::beers::Beer, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        // Check ownership and update
        let beer = sqlx::query_as!(
            crate::definitions::beers::Beer,
            "UPDATE beers SET name = $1, brewery = $2, type = $3, abv = $4, ibu = $5, color = $6, image_url = $7 
             WHERE id = $8 AND created_by = $9
             RETURNING id, name, brewery, type as \"type\", abv, ibu, color, image_url, created_by, created_at",
            input.name,
            input.brewery,
            input.r#type,
            input.abv,
            input.ibu,
            input.color,
            input.image_url,
            beer_id,
            auth_user.id
        )
        .fetch_optional(pool)
        .await?
        .ok_or("Bier niet gevonden of je hebt geen rechten")?;

        Ok(beer)
    }

    async fn create_club(
        &self,
        ctx: &Context<'_>,
        input: crate::definitions::clubs::CreateClubInput,
    ) -> Result<crate::definitions::clubs::Club, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        let slug = input.name.to_lowercase().replace(" ", "-"); // Simple slug
        
        // Transaction: Create Club + Add Owner Membership
        let mut tx = pool.begin().await.map_err(|_| "Transaction start failed")?;

        let club = sqlx::query_as!(
            crate::definitions::clubs::Club,
            "INSERT INTO clubs (name, slug, description, owner_id, image_url) VALUES ($1, $2, $3, $4, $5) RETURNING id, name, slug, description, owner_id, image_url, image_path, created_at",
            input.name,
            slug,
            input.description,
            auth_user.id,
            input.image_url
        )
        .fetch_one(&mut *tx)
        .await?;

        // Add owner membership
        sqlx::query!(
            "INSERT INTO club_memberships (club_id, user_id, role, status) VALUES ($1, $2, 'OWNER'::user_role, 'ACTIVE'::member_status)",
            club.id,
            auth_user.id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await.map_err(|_| "Transaction commit failed")?;

        Ok(club)
    }

    async fn join_club(
        &self,
        ctx: &Context<'_>,
        club_id: i32,
    ) -> Result<bool, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        sqlx::query!(
            "INSERT INTO club_memberships (club_id, user_id, role, status) VALUES ($1, $2, 'MEMBER'::user_role, 'ACTIVE'::member_status) ON CONFLICT DO NOTHING",
            club_id,
            auth_user.id
        )
        .execute(pool)
        .await?;

        Ok(true)
    }

    async fn send_message(
        &self,
        ctx: &Context<'_>,
        input: crate::definitions::chat::SendMessageInput,
    ) -> Result<crate::definitions::chat::ClubMessage, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        // Encrypt content
        let encrypted_content = crypto::encrypt_string(&input.content)?;

        let message = sqlx::query_as!(
            crate::definitions::chat::ClubMessage,
            "INSERT INTO club_messages (club_id, user_id, content) VALUES ($1, $2, $3) RETURNING id, club_id as \"club_id!\", user_id, content, created_at, NULL::text as user_display_name, NULL::text as user_avatar_url",
            input.club_id,
            auth_user.id,
            encrypted_content
        )
        .fetch_one(pool)
        .await?;

        let mut result_msg = message;
        result_msg.content = input.content; 
        
        Ok(result_msg)
    }

    async fn create_event(
        &self,
        ctx: &Context<'_>,
        input: crate::definitions::events::CreateEventInput,
    ) -> Result<crate::definitions::events::Event, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        let event = sqlx::query_as!(
            crate::definitions::events::Event,
            "INSERT INTO events (club_id, title, description, location, starts_at, ends_at, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, club_id, title, description, location, starts_at, ends_at, created_by, created_at",
            input.club_id,
            input.title,
            input.description,
            input.location,
            input.starts_at,
            input.ends_at,
            auth_user.id
        )
        .fetch_one(pool)
        .await?;

        Ok(event)
    }

    async fn rsvp_event(
        &self,
        ctx: &Context<'_>,
        event_id: i32,
        status: crate::definitions::events::RsvpStatus,
    ) -> Result<bool, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>().map_err(|_| "Database pool missing")?;
        let auth_user = ctx.data::<crate::AuthUser>().map_err(|_| "Niet ingelogd")?;

        // Need to cast enum again?
        // rsvp_status enum in DB: 'GOING', 'INTERESTED', 'NOT_GOING'
        // Rust enum RsvpStatus::Going -> "GOING" (via rename_all)
        // sqlx::Type should handle it if passed as param.
        // Let's rely on sqlx type mapping this time as we defined it with #[derive(sqlx::Type)].
        
        sqlx::query!(
            "INSERT INTO event_attendees (event_id, user_id, status) VALUES ($1, $2, $3) 
             ON CONFLICT (event_id, user_id) DO UPDATE SET status = $3",
            event_id,
            auth_user.id,
            status as crate::definitions::events::RsvpStatus 
        )
        .execute(pool)
        .await?;

        Ok(true)
    }
}
