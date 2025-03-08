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
        .expect("DATABASE_URL must be set");

    // Configure connection manager with proper settings for PgBouncer
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    // Create connection pool with PgBouncer-compatible settings
    let pool = r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(3))
        .max_size(10) // Increase max connections for API
        .min_idle(Some(1)) // Keep at least one connection ready
        .idle_timeout(Some(Duration::from_secs(10))) // Release connections after 10 seconds idle
        .build(manager)
        .expect("Failed to create connection pool");
    
    // Create some fake users at startup
    create_fake_users(&pool).expect("Failed to create fake users");
    
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
    
    // Create a TCP listener
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", addr);
    
    // Run the server with axum's serve method
    axum::serve(listener, app).await.unwrap();
}

// Create initial fake users
fn create_fake_users(pool: &DbPool) -> Result<(), diesel::result::Error> {
    let mut conn = pool.get().expect("Failed to get a connection");
    
    // Generate 10 fake users
    let fake_users = NewUser::generate_fake_batch(10);
    
    // Insert users in batches
    for user_batch in fake_users.chunks(5) {
        diesel::insert_into(users)
            .values(user_batch)
            .execute(&mut conn)?;
    }
    
    // Log the created users
    let results = users.load::<User>(&mut conn)?;
    
    tracing::info!("Created {} fake users", results.len());
    
    Ok(())
}

// Handler for GET /users
async fn get_users(State(pool): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let conn = &mut pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let result = users.load::<User>(conn)
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
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
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let result = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(result))
}

// Health check endpoint
async fn health_check() -> StatusCode {
    StatusCode::OK
}