use std::fmt;

use serde::{Deserialize, Deserializer, de::{self, MapAccess, Visitor}};

#[derive(Debug, Clone, Deserialize)]
pub struct ClientConfig
{
    pub server_root: String,
    pub credentials: Credentials,
}

#[derive(Debug, Clone)]
pub enum Credentials 
{
    Password(String),
    SessionId(String),
}

impl ClientConfig
{
    pub fn from_file(file_path: &str) -> Result<ClientConfig, InitError> 
    {
        let json_data = std::fs::read_to_string(file_path)?; 

        let config: ClientConfig = serde_json::from_str(&json_data)?;

        Ok(config)
    }
}

#[derive(Debug)]
pub enum InitError
{
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for InitError 
{
    fn from(err: std::io::Error) -> Self 
    {
        InitError::Io(err)
    }
}

impl From<serde_json::Error> for InitError 
{
    fn from(err: serde_json::Error) -> Self 
    {
        InitError::Json(err)
    }
}

impl std::fmt::Display for InitError 
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        write!(f, "{:?}", self)
    }
}

impl<'de> Deserialize<'de> for Credentials
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Password,
            SessionId,
        }

        impl<'de> Deserialize<'de> for Field
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                match s.as_str() {
                    "password" => Ok(Field::Password),
                    "session_id" => Ok(Field::SessionId),
                    _ => Err(de::Error::unknown_field(&s, &["password", "session_id"])),
                }
            }
        }

        struct CredentialsVisitor;

        impl<'de> Visitor<'de> for CredentialsVisitor
        {
            type Value = Credentials;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result
            {
                f.write_str("password or session_id")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let key: Field = map.next_key()?.ok_or_else(|| de::Error::custom("empty credentials"))?;
                let value: String = map.next_value()?;

                match key {
                    Field::Password => Ok(Credentials::Password(value)),
                    Field::SessionId => Ok(Credentials::SessionId(value)),
                }
            }
        }

        deserializer.deserialize_map(CredentialsVisitor)
    }
}