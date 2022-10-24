use num_bigint::BigInt;
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(bi: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&crate::big_int_to_lower_hex(bi))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    crate::from_hex_to_big_int(&s).map_err(serde::de::Error::custom)
}
