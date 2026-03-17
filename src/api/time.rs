use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub hour: u32,   // 0-23
    pub minute: u32, // 0-59
}

impl Time 
{
    pub fn compute_duration(&self, other: Time) -> Duration {
        // Convert both times to minutes
        let self_minutes = self.hour as i32 * 60 + self.minute as i32;
        let other_minutes = other.hour as i32 * 60 + other.minute as i32;

        // Calculatepub  difference (wrap around 24h if negative)
        let mut diff_minutes = other_minutes - self_minutes;
        if diff_minutes < 0 {
            diff_minutes += 24 * 60; // wrap to next day
        }

        Duration::from_secs((diff_minutes * 60) as u64)

    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let s = format!("{:02}:{:02}", self.hour, self.minute);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Time 
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateVisitor;

        impl<'de> Visitor<'de> for DateVisitor {
            type Value = Time;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a date string in hh:mm format")
            }

            fn visit_str<E>(self, v: &str) -> Result<Time, E>
            where
                E: de::Error,
            {
                let parts: Vec<&str> = v.split('/').collect();

                if parts.len() != 3 {
                    return Err(E::custom("invalid date format"));
                }

                let hour  = parts[0].parse().map_err(E::custom)?;
                let minute = parts[1].parse().map_err(E::custom)?;

                Ok(Time { hour, minute })
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}