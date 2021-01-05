use crate::errors::Error;

/// Some fixed sized arrays don't implement Default trait in standard library. Since we can't
/// implement a trait outside of crate, we created a new trait
pub trait DefaultFrom {
    fn default() -> Self;
}

pub trait FromBytes {
    fn from_bytes(data: &[u8]) -> Result<&Self, Error>;
}
