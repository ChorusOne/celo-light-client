pub(crate) mod hexstring {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    use hex::FromHex;

    /// Deserialize string into T
    pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: hex::FromHex,
        <T as FromHex>::Error: std::fmt::Display
    {
    	let s: &str = Deserialize::deserialize(deserializer)?;
        if s.len() <= 2 || !s.starts_with("0x"){
            return Err(D::Error::custom(format!("hex string should start with '0x', got: {}", s)));
        }

	T::from_hex(&s[2..]).map_err(D::Error::custom)
    }
    
    /// Serialize from T into string
    pub(crate) fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<[u8]>,
    {
        let prefix = "0x".to_string();
        let hex_string = hex::encode(value.as_ref());

        serializer.serialize_str(&(prefix + &hex_string))
    }
}

pub(crate) mod hexnum {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use num;

    /// Deserialize string into T
    pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: num::traits::Num,
        <T as num::Num>::FromStrRadixErr: std::fmt::Display
    {
    	let s: &str = Deserialize::deserialize(deserializer)?;
        if s.len() <= 2 || !s.starts_with("0x"){
            return Err(D::Error::custom(format!("hex string should start with '0x', got: {}", s)));
        }
	T::from_str_radix(&s[2..], 16).map_err(D::Error::custom)
    }
    
    /// Serialize from T into string
    pub(crate) fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: std::fmt::LowerHex,
    {
        format!("0x{:x}", value).serialize(serializer)
    }
}
