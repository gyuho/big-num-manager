use num_bigint::BigInt;
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(bi: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // ref. https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.to_rfc3339_opts
    serializer.serialize_str(&crate::big_int_to_lower_hex(bi))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // ref. https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.parse_from_rfc3339
    crate::from_hex_to_big_int(&s).map_err(serde::de::Error::custom)
}
