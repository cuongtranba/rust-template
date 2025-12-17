//! Application configuration
//!
//! Handles loading configuration from environment variables and files.
//! Uses the `config` crate for flexible configuration management.
//!
//! ## Configuration Sources (in order of precedence)
//!
//! 1. Environment variables (prefixed with `APP_`)
//! 2. `.env` file (if present)
//! 3. `config/local.toml` (for local development)
//! 4. `config/default.toml` (base configuration)
//!
//! ## Environment Variables
//!
//! - `APP_ENVIRONMENT`: `development`, `staging`, `production`
//! - `APP_LOG_LEVEL`: `trace`, `debug`, `info`, `warn`, `error`
//! - `APP_DATABASE_URL`: Database connection string
//! - `APP_SERVER_HOST`: Server host (default: `127.0.0.1`)
//! - `APP_SERVER_PORT`: Server port (default: `3000`)

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// Application configuration
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    /// Current environment
    #[serde(default = "default_environment")]
    pub environment: String,

    /// Logging configuration
    #[serde(default)]
    pub log: LogConfig,

    /// Server configuration (for HTTP adapter)
    #[serde(default)]
    pub server: ServerConfig,

    /// Database configuration
    #[serde(default)]
    pub database: DatabaseConfig,
}

/// Logging configuration
#[derive(Debug, Deserialize, Clone)]
pub struct LogConfig {
    /// Log level
    #[serde(default = "default_log_level")]
    pub level: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
        }
    }
}

/// Server configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    /// Server host
    #[serde(default = "default_host")]
    pub host: String,

    /// Server port
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

impl ServerConfig {
    /// Get the server address as a string
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Database configuration
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL
    #[serde(default)]
    pub url: String,

    /// Maximum number of connections in the pool
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            max_connections: default_max_connections(),
        }
    }
}

// Default value functions
fn default_environment() -> String {
    "development".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_max_connections() -> u32 {
    5
}

impl AppConfig {
    /// Load configuration from files and environment
    ///
    /// # Configuration Sources (in order)
    ///
    /// 1. `config/default.toml` - Base configuration
    /// 2. `config/{environment}.toml` - Environment-specific config
    /// 3. `config/local.toml` - Local overrides (not in git)
    /// 4. Environment variables prefixed with `APP_`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let config = AppConfig::load()?;
    /// println!("Running on: {}", config.server.address());
    /// ```
    pub fn load() -> Result<Self, ConfigError> {
        let environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // Start with default values
            .set_default("environment", environment.clone())?
            // Load base config file
            .add_source(File::with_name("config/default").required(false))
            // Load environment-specific config
            .add_source(File::with_name(&format!("config/{}", environment)).required(false))
            // Load local config (for development overrides)
            .add_source(File::with_name("config/local").required(false))
            // Override with environment variables
            // e.g., `APP_SERVER_PORT=8080` sets `server.port`
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("_")
                    .try_parsing(true),
            )
            .build()?;

        config.try_deserialize()
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        // This test verifies defaults work without any config files
        let config = AppConfig {
            environment: default_environment(),
            log: LogConfig::default(),
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
        };

        assert_eq!(config.environment, "development");
        assert_eq!(config.log.level, "info");
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.server.address(), "127.0.0.1:3000");
    }

    #[test]
    fn test_is_development() {
        let config = AppConfig {
            environment: "development".to_string(),
            log: LogConfig::default(),
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
        };

        assert!(config.is_development());
        assert!(!config.is_production());
    }

    #[test]
    fn test_is_production() {
        let config = AppConfig {
            environment: "production".to_string(),
            log: LogConfig::default(),
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
        };

        assert!(!config.is_development());
        assert!(config.is_production());
    }
}
