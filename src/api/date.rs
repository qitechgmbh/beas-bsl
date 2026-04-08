use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
} // 2020/02/28

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:04}/{:02}/{:02}", self.year, self.month, self.day);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateVisitor;

        impl<'de> Visitor<'de> for DateVisitor {
            type Value = Date;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a date string in YYYY/MM/DD format")
            }

            fn visit_str<E>(self, v: &str) -> Result<Date, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = v.split('/').collect();

                if parts.len() != 3 {
                    return Err(E::custom("invalid date format"));
                }

                let year = parts[0].parse().map_err(E::custom)?;
                let month = parts[1].parse().map_err(E::custom)?;
                let day = parts[2].parse().map_err(E::custom)?;

                Ok(Date { year, month, day })
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}
