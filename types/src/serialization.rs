pub(crate) mod hexstring {
    use hex::FromHex;
    use serde::{de::Error, Deserialize, Deserializer, Serializer};

    /// Deserialize string into T
    pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: hex::FromHex,
        <T as FromHex>::Error: std::fmt::Display,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let stripped = s.strip_prefix("0x").unwrap_or(&s);
        T::from_hex(stripped).map_err(D::Error::custom)
    }

    /// Serialize from T into string
    pub(crate) fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let hex_string = hex::encode(value.as_ref());
        if hex_string.is_empty() {
            return serializer.serialize_str("");
        }

        serializer.serialize_str(&(String::from("0x") + &hex_string))
    }
}
