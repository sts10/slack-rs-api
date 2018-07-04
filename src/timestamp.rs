use serde::de::{self, Deserialize, Deserializer, Error, Visitor};
use std::fmt;

// NOTE: This falls apart after a few decades
#[derive(Clone, Copy, Debug)]
pub struct Timestamp {
    seconds: u32,
    milliseconds: Option<[u8; 3]>,
}

struct TimestampVisitor;

impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Unix-style timestamp, as a u32 or string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Timestamp, E>
    where
        E: de::Error,
    {
        println!("{}", value.len());
        if value.len() <= 17 {
            // Split at the decimal point
            let dot_location = value
                .find('.')
                .ok_or_else(|| Error::custom("Got a string without a ."))?;
            let (seconds_str, millis_str) = value.split_at(dot_location);
            Ok(Timestamp {
                seconds: seconds_str
                    .parse()
                    .map_err(|_| Error::custom(format!("Cannot parse {} as a number", seconds_str)))?,
                milliseconds: Some(u32_to_u8s(
                    millis_str[1..]
                        .parse()
                        .map_err(|_| Error::custom(format!("Cannot parse {} as a number", millis_str)))?,
                )),
            })
        } else {
            Err(E::custom(format!(
                "Timestamps must be string or number with 16 decimal places, got {}",
                value
            )))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Timestamp, E>
    where
        E: de::Error,
    {
        Ok(Timestamp {
            seconds: value as u32,
            milliseconds: None,
        })
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TimestampVisitor)
    }
}

fn u32_to_u8s(x: u32) -> [u8; 3] {
    let b1: u8 = ((x >> 16) & 0xff) as u8;
    let b2: u8 = ((x >> 8) & 0xff) as u8;
    let b3: u8 = (x & 0xff) as u8;
    return [b1, b2, b3];
}
