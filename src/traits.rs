use crate::errors::Error;

/// Some fixed sized arrays don't implement Default trait in standard library. Since we can't
/// implement a trait outside of crate, we created a new trait
pub trait DefaultFrom {
    fn default() -> Self;
}

pub trait FromBytes {
    fn from_bytes(data: &[u8]) -> Result<&Self, Error>;
}

pub trait Storage {
    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>, Error>;
    fn get(&self, key: &[u8]) -> Result<Vec<u8>, Error>;
    fn contains_key(&self, key: &[u8]) -> Result<bool, Error>;
}
