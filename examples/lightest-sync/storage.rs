extern crate sled;

use sled::Db;
use celo::Error;
use celo::Kind;
use celo::Storage;

pub struct ExampleStorage {
    db: Db,
}

impl ExampleStorage {
    pub fn new(path: &str) -> Self {
        Self {
            db: sled::open(path).expect("open")
        }
    }
}

impl Storage for ExampleStorage {
    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        match self.db.insert(key, value) {
            Ok(value) => Ok(Some(value.unwrap_or_default().to_vec())),
            Err(e) => Err(Kind::InvalidChainInsertion.into()), // TODO
        }
    }

    fn get(&self, key: &[u8]) -> Result<Vec<u8>, Error> {
        let result = self.db.get(key);
        Ok(
            sled::IVec::from(
                &result.unwrap_or_default().unwrap_or_default()
            ).to_vec()
        )
    }

    fn contains_key(&self, key: &[u8]) -> Result<bool, Error> {
       match self.db.contains_key(key) {
           Ok(value) => Ok(value),
           Err(e) => Err(Kind::InvalidChainInsertion.into()), // TODO
       }
    }
}
