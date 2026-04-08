mod credentials;
mod error;

pub use credentials::Credentials;
pub use error::ConfigError;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub server_root: String,
    pub credentials: Credentials,
}

impl Config {
    pub fn new(server_root: String, credentials: Credentials) -> Self {
        Self {
            server_root,
            credentials,
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self, ConfigError> {
        let json_data = std::fs::read_to_string(file_path)?;

        let config: Config = serde_json::from_str(&json_data)?;

        Ok(config)
    }
}
