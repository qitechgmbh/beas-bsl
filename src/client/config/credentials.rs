use std::fmt;

use serde::{
    Deserialize, Deserializer,
    de::{self, MapAccess, Visitor},
};

#[derive(Debug, Clone)]
pub enum Credentials {
    Password(String),
    SessionId(String),
}

impl<'de> Deserialize<'de> for Credentials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Password,
            SessionId,
        }

        impl<'de> Deserialize<'de> for Field {
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

        impl<'de> Visitor<'de> for CredentialsVisitor {
            type Value = Credentials;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("password or session_id")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let key: Field = map
                    .next_key()?
                    .ok_or_else(|| de::Error::custom("empty credentials"))?;
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
