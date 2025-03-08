use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Router, Json,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::net::TcpListener;

pub mod models;
pub mod schema;

use models::{NewUser, User};
use schema::users::dsl::*;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
type AppState = Arc<DbPool>;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get the database URL from the environment
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            tracing::error!("DATABASE_URL environment variable not set");
            "postgres://db_user:db_password@pgbouncer:6432/demo_db".to_string()
        });

    // Configure connection manager with proper settings for PgBouncer
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    // Create connection pool with PgBouncer-compatible settings
    let pool = match r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(3))
        .max_size(10) // Increase max connections for API
        .min_idle(Some(1)) // Keep at least one connection ready
        .idle_timeout(Some(Duration::from_secs(10))) // Release connections after 10 seconds idle
        .build(manager) {
            Ok(pool) => pool,
            Err(e) => {
                tracing::error!("Failed to create connection pool: {}", e);
                std::process::exit(1);
            }
        };
    
    // Create some fake users at startup - handle errors gracefully
    if let Err(e) = create_fake_users(&pool) {
        tracing::error!("Failed to create fake users: {}", e);
        // Continue running the server even if fake user creation fails
    }
    
    // Create a shared state that holds the database pool
    let state = Arc::new(pool);
    
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Build our application with routes
    let app = Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/health", get(health_check))
        .layer(cors)
        .with_state(state);

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting server on {}", addr);
    
    // Create a TCP listener with error handling
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            tracing::info!("Listening on {}", addr);
            listener
        },
        Err(e) => {
            tracing::error!("Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };
    
    // Run the server with axum's serve method
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
    }
}

// Create initial fake users
fn create_fake_users(pool: &DbPool) -> Result<(), diesel::result::Error> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get a connection from pool: {}", e);
            return Ok(()); // Return Ok to prevent server from stopping
        }
    };
    
    // Generate 10 fake users
    let fake_users = NewUser::generate_fake_batch(10);
    
    // Insert users in batches with error handling for each batch
    for user_batch in fake_users.chunks(5) {
        match diesel::insert_into(users)
            .values(user_batch)
            .execute(&mut conn) {
                Ok(_) => {},
                Err(e) => {
                    tracing::error!("Failed to insert user batch: {}", e);
                    // Continue with next batch instead of returning error
                    continue;
                }
            };
    }
    
    // Log the created users
    match users.load::<User>(&mut conn) {
        Ok(results) => {
            tracing::info!("Created {} fake users", results.len());
        },
        Err(e) => {
            tracing::error!("Failed to load users after creation: {}", e);
        }
    }
    
    Ok(())
}

// Handler for GET /users
async fn get_users(State(pool): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let conn = &mut pool
        .get()
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let result = users.load::<User>(conn)
        .map_err(|e| {
            tracing::error!("Database error when loading users: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(result))
}

// Handler for POST /users
async fn create_user(
    State(pool): State<AppState>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let conn = &mut pool
        .get()
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let result = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
        .map_err(|e| {
            tracing::error!("Database error when creating user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(result))
}

// Health check endpoint
async fn health_check() -> StatusCode {
    StatusCode::OK
}