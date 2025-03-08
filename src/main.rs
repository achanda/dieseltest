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
    // Application startup
    tracing::info!("Starting Rust API application...");
    
    // Initialize tracing
    tracing::info!("Initializing logging system...");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                tracing::info!("RUST_LOG not set, defaulting to 'info'");
                "info".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("Logging system initialized successfully");

    // Get the database URL from the environment
    tracing::info!("Retrieving DATABASE_URL from environment...");
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            tracing::error!("DATABASE_URL environment variable not set");
            tracing::info!("Using default connection string for PgBouncer");
            "postgres://db_user:db_password@pgbouncer:6432/demo_db".to_string()
        });
    tracing::info!("Using database URL: {}", database_url);

    // Configure connection manager with proper settings for PgBouncer
    tracing::info!("Creating database connection manager...");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    tracing::info!("Connection manager created successfully");
    
    // Create connection pool with PgBouncer-compatible settings
    tracing::info!("Building database connection pool with PgBouncer settings...");
    let pool = match r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(3))
        .max_size(10) // Increase max connections for API
        .min_idle(Some(1)) // Keep at least one connection ready
        .idle_timeout(Some(Duration::from_secs(10))) // Release connections after 10 seconds idle
        .build(manager) {
            Ok(pool) => {
                tracing::info!("Connection pool created successfully with max_size=10, min_idle=1");
                pool
            },
            Err(e) => {
                tracing::error!("Failed to create connection pool: {}", e);
                tracing::error!("Application cannot start without database connection");
                std::process::exit(1);
            }
        };
    
    // Test database connection with a simple SELECT 1 query
    tracing::info!("Testing database connection with SELECT 1 query...");
    match test_database_connection(&pool) {
        Ok(_) => tracing::info!("Database connection test successful (SELECT 1)"),
        Err(e) => tracing::warn!("Database connection test failed: {}", e),
    }
    
    // Create some fake users at startup - handle errors gracefully
    tracing::info!("Attempting to create fake users for testing...");
    if let Err(e) = create_fake_users(&pool) {
        tracing::error!("Failed to create fake users: {}", e);
        tracing::info!("Continuing application startup despite fake user creation failure");
        // Continue running the server even if fake user creation fails
    }
    
    // Create a shared state that holds the database pool
    tracing::info!("Creating shared application state with database pool...");
    let state = Arc::new(pool);
    tracing::info!("Application state created successfully");
    
    // Configure CORS
    tracing::info!("Configuring CORS policy...");
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    tracing::info!("CORS configured to allow any origin, method, and header");
    
    // Build our application with routes
    tracing::info!("Building application router with routes...");
    let app = Router::new()
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/health", get(health_check))
        .layer(cors)
        .with_state(state);
    tracing::info!("Router configured with /users GET/POST and /health endpoints");

    // Run the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Preparing to start server on {}", addr);
    
    // Create a TCP listener with error handling
    tracing::info!("Binding to address {}...", addr);
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            tracing::info!("Successfully bound to {}", addr);
            listener
        },
        Err(e) => {
            tracing::error!("Failed to bind to {}: {}", addr, e);
            tracing::error!("Cannot start server without binding to port");
            std::process::exit(1);
        }
    };
    
    // Run the server with axum's serve method
    tracing::info!("Starting HTTP server... 🚀");
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
        tracing::error!("Server has stopped due to an error");
    } else {
        tracing::info!("Server has stopped gracefully");
    }
}

// Function to test database connection with a simple SELECT 1 query
fn test_database_connection(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Acquiring connection from pool for SELECT 1 test...");
    let mut conn = pool.get()?;
    tracing::info!("Connection acquired, executing SELECT 1...");
    
    // Execute a simple SELECT 1 query to test the connection
    diesel::sql_query("SELECT 1 as result")
        .execute(&mut conn)?;
    
    tracing::info!("SELECT 1 query executed successfully");
    Ok(())
}

// Create initial fake users
fn create_fake_users(pool: &DbPool) -> Result<(), diesel::result::Error> {
    tracing::info!("Starting fake user creation process");
    
    let mut conn = match pool.get() {
        Ok(conn) => {
            tracing::info!("Successfully acquired database connection for fake user creation");
            conn
        },
        Err(e) => {
            tracing::error!("Failed to get a connection from pool: {}", e);
            tracing::warn!("Skipping fake user creation due to connection error");
            return Ok(()); // Return Ok to prevent server from stopping
        }
    };
    
    // Generate 10 fake users
    tracing::info!("Generating 10 fake user records...");
    let fake_users = NewUser::generate_fake_batch(10);
    tracing::info!("Successfully generated 10 fake user records in memory");
    
    // Insert users in batches with error handling for each batch
    tracing::info!("Inserting fake users in batches of 5...");
    for (i, user_batch) in fake_users.chunks(5).enumerate() {
        tracing::info!("Processing batch {} with {} users", i+1, user_batch.len());
        match diesel::insert_into(users)
            .values(user_batch)
            .execute(&mut conn) {
                Ok(count) => {
                    tracing::info!("Successfully inserted {} users in batch {}", count, i+1);
                },
                Err(e) => {
                    tracing::error!("Failed to insert user batch {}: {}", i+1, e);
                    tracing::info!("Continuing with next batch instead of returning error");
                    continue;
                }
            };
    }
    
    // Log the created users
    tracing::info!("Attempting to count total users in database...");
    match users.load::<User>(&mut conn) {
        Ok(results) => {
            tracing::info!("Successfully created {} fake users in total", results.len());
        },
        Err(e) => {
            tracing::error!("Failed to load users after creation: {}", e);
            tracing::warn!("User creation may have succeeded but count verification failed");
        }
    }
    
    tracing::info!("Fake user creation process completed");
    Ok(())
}

// Handler for GET /users
async fn get_users(State(pool): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    tracing::info!("Handling GET /users request");
    
    let conn = &mut pool
        .get()
        .map_err(|e| {
            tracing::error!("Failed to get database connection for GET /users: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    tracing::info!("Successfully acquired database connection for GET /users");
    
    let result = users.load::<User>(conn)
        .map_err(|e| {
            tracing::error!("Database error when loading users: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    tracing::info!("Successfully retrieved {} users from database", result.len());
    Ok(Json(result))
}

// Handler for POST /users
async fn create_user(
    State(pool): State<AppState>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    tracing::info!("Handling POST /users request with user: {:?}", new_user);
    
    let conn = &mut pool
        .get()
        .map_err(|e| {
            tracing::error!("Failed to get database connection for POST /users: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    tracing::info!("Successfully acquired database connection for POST /users");
    
    let result = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
        .map_err(|e| {
            tracing::error!("Database error when creating user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    tracing::info!("Successfully created new user with id: {}", result.id);
    Ok(Json(result))
}

// Health check endpoint
async fn health_check() -> StatusCode {
    tracing::info!("Health check requested");
    StatusCode::OK
}