use chrono::{DateTime, SecondsFormat};
// serde::ser::Error as that's the trait where the custom function comes from.
use serde::{de::Error as DeError, ser::Error as SerError, Deserialize, Deserializer, Serializer};

pub fn deserialize_time<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let input = <&str>::deserialize(deserializer)?;
    match DateTime::parse_from_rfc3339(input) {
        Ok(ok) => Ok(ok.timestamp_millis()),
        Err(err) => Err(D::Error::custom(err)),
    }
}

pub fn serialize_time<S>(x: &i64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let Some(time) = DateTime::from_timestamp_millis(*x) else {
        return Err(S::Error::custom("timestamp out of range"));
    };
    s.serialize_str(&time.to_rfc3339_opts(SecondsFormat::Millis, true))
}

pub fn deserialize_time_optional<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let input = <Option<&str>>::deserialize(deserializer)?;
    let Some(input) = input else { return Ok(None) };
    match DateTime::parse_from_rfc3339(input) {
        Ok(ok) => Ok(Some(ok.timestamp_millis())),
        Err(err) => Err(D::Error::custom(err)),
    }
}

pub fn serialize_time_optional<S>(x: &Option<i64>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        Some(x) => {
            let Some(time) = DateTime::from_timestamp_millis(*x) else {
                return Err(S::Error::custom("timestamp out of range"));
            };
            s.serialize_str(&time.to_rfc3339_opts(SecondsFormat::Millis, true))
        }
        None => s.serialize_none(),
    }
}

pub fn default_true() -> bool {
    true
}

pub fn default_false() -> bool {
    false
}
