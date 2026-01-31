//███╗░░░███╗░█████╗░██████╗░██╗░░░██╗██╗░░░░░███████╗░██████╗
//████╗░████║██╔══██╗██╔══██╗██║░░░██║██║░░░░░██╔════╝██╔════╝
//██╔████╔██║██║░░██║██║░░██║██║░░░██║██║░░░░░█████╗░░╚█████╗░
//██║╚██╔╝██║██║░░██║██║░░██║██║░░░██║██║░░░░░██╔══╝░░░╚═══██╗
//██║░╚═╝░██║╚█████╔╝██████╔╝╚██████╔╝███████╗███████╗██████╔╝
//╚═╝░░░░░╚═╝░╚════╝░╚═════╝░░╚═════╝░╚══════╝╚══════╝╚═════╝░

mod definitions;
mod schema;
mod utils;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use axum::{
    response::{self, IntoResponse},
    routing::{get, post},
    Router,
    Extension,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::schema::{Query, Mutation, BierSchema};
use crate::utils::auth::verify_jwt;
use sqlx::types::Uuid;

pub struct AuthUser {
    pub id: i32,
    pub session_id: Uuid,
}

pub struct RequestMetadata {
    pub user_agent: Option<String>,
}

async fn graphql_handler(
    Extension(schema): Extension<BierSchema>,
    Extension(pool): Extension<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    let metadata = RequestMetadata {
        user_agent: headers.get("user-agent").and_then(|h| h.to_str().ok().map(|s| s.to_string())),
    };
    req = req.data(metadata);

    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if let Ok(claims) = verify_jwt(token) {
                    if let Ok(sid_uuid) = Uuid::parse_str(&claims.sid) {
                        let session_exists = sqlx::query!(
                            "SELECT id FROM sessions WHERE id = $1 AND expires_at > NOW()",
                            sid_uuid
                        )
                        .fetch_optional(&pool)
                        .await
                        .ok()
                        .flatten();

                        if session_exists.is_some() {
                            if let Ok(user_id) = claims.sub.parse::<i32>() {
                                req = req.data(AuthUser { 
                                    id: user_id,
                                    session_id: sid_uuid,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    schema.execute(req).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    tracing::info!("Starting in {} mode", app_env);

    let database_url = if app_env == "production" {
        std::env::var("DATABASE_URL_PROD").expect("DATABASE_URL_PROD must be set in production")
    } else {
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env")
    };
    
    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to to database");

    tracing::info!("✅ Database connected successfully!");

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool.clone())
        .finish();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/", get(graphql_playground))
        .layer(Extension(schema))
        .layer(Extension(pool.clone()))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
