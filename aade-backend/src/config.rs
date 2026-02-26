use std::env;

pub struct Config {
    pub database_url: String,
    pub server_addr: String,
    pub cors_allowed_origins: Vec<String>,
    pub environment: Environment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let env_str = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let environment = match env_str.to_lowercase().as_str() {
            "production" | "prod" => Environment::Production,
            _ => Environment::Development,
        };

        // Parse CORS origins from comma-separated list
        let cors_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| {
                if environment == Environment::Production {
                    // In production, no default - must be explicitly set
                    String::new()
                } else {
                    // In development, allow common local ports
                    "http://localhost:3000,http://localhost:5173,http://localhost:5174".to_string()
                }
            });

        let cors_allowed_origins: Vec<String> = cors_origins
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| format!("0.0.0.0:{}", port));

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_addr,
            cors_allowed_origins,
            environment,
        }
    }
}
