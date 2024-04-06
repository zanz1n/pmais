#[cfg(not(feature = "bson-kv"))]
pub fn serialize<T: serde::Serialize>(value: &T) -> Option<Vec<u8>> {
    match serde_json::to_vec(value) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

#[cfg(feature = "bson-kv")]
pub fn serialize<T: serde::Serialize>(value: &T) -> Option<Vec<u8>> {
    match bson::to_vec(value) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

#[cfg(not(feature = "bson-kv"))]
pub fn deserialize<'de, T: serde::Deserialize<'de>>(buf: &'de [u8]) -> Option<T> {
    match serde_json::from_slice(buf) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

#[cfg(feature = "bson-kv")]
pub fn deserialize<'de, T: serde::Deserialize<'de>>(buf: &'de [u8]) -> Option<T> {
    match bson::from_slice(buf) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}
