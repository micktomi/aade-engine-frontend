use axum::{
    routing::{get, post},
    Router,
    extract::DefaultBodyLimit,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use sqlx::postgres::PgPoolOptions;
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
    key_extractor::SmartIpKeyExtractor,
};

use aade_validator::api;
use aade_validator::config::Config;
use aade_validator::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();

    // Setup database (optional - only for logging)
    let pool = match tokio::time::timeout(
        std::time::Duration::from_secs(2),
        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect(&config.database_url)
    ).await {
        Ok(Ok(pool)) => {
            // Run migrations if DB is available
            if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                tracing::warn!("Database migrations failed: {}. Continuing without DB logging.", e);
                None
            } else {
                tracing::info!("Database connected - validation logging enabled");
                Some(pool)
            }
        }
        Ok(Err(e)) => {
            tracing::warn!("Database connection error: {}. Continuing without DB logging.", e);
            None
        }
        Err(_) => {
            tracing::warn!("Database connection timeout (2s). Continuing without DB logging.");
            None
        }
    };

    let state = AppState { db: pool };

    // Rate Limiting: 100 requests per 15 minutes per IP
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(1)  // Avg rate: 1 req/sec
            .burst_size(100) // Allow burst of 100 requests
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .unwrap()
    );
    let rate_limiter = GovernorLayer {
        config: governor_conf,
    };

    tracing::info!("Rate limiting enabled: 100 requests/15min per IP (burst: 100)");

    // CORS: Configure based on environment
    let cors = if config.environment == aade_validator::config::Environment::Production {
        // Production: Strict CORS with explicit origins
        if config.cors_allowed_origins.is_empty() {
            tracing::warn!("Production mode but no CORS_ALLOWED_ORIGINS set - no origins will be allowed");
        }

        let origins: Vec<axum::http::HeaderValue> = config.cors_allowed_origins
            .iter()
            .filter_map(|origin| origin.parse().ok())
            .collect();

        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
            .allow_headers([axum::http::header::CONTENT_TYPE])
    } else {
        // Development: Permissive CORS for easier testing
        tracing::info!("Development mode - using permissive CORS");
        CorsLayer::permissive()
    };

    let app = Router::new()
        .route("/health/ready", get(api::health::readiness))
        .route("/health/live", get(api::health::liveness))
        .route("/validate", post(api::validate::validate_invoice))
        .route("/validate/batch", post(api::validate::validate_batch))
        .with_state(state)
        .layer(rate_limiter)  // Rate limiting layer
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)); // 10MB limit

    let addr: SocketAddr = config.server_addr.parse()?;
    tracing::info!("AADE Validator listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    ).await?;

    Ok(())
}
