
use crate::types::header::{Address};
use crate::types::istanbul::{SerializedPublicKey};
use rug::Integer;
use std::collections::HashMap;
use crate::types::header::Hash;

#[derive(Debug, Clone, Copy)]
pub struct Validator{
    pub address: Address,
    pub public_key: SerializedPublicKey,
}

#[derive(Debug, Clone)]
pub struct State {
    pub validators: Vec<Validator>, // Set of authorized validators at this moment
    pub epoch: u64, // the number of blocks for each epoch
    pub number: u64, // block number where the snapshot was created
    pub hash: Hash, // block hash where the snapshot was created
}

impl State {
    pub fn new() -> Self {
        State {
            validators: Vec::new(),
            epoch: 0,
            number: 0,
            hash: Hash::default(),
        }
    }

    pub fn add_validators(&mut self, validators: Vec<Validator>) -> bool {
        let mut new_address_map: HashMap<Address, bool> = HashMap::new();
        
        for validator in validators.iter() {
            new_address_map.insert(validator.address, true);
        }

        // Verify that the validators to add is not already in the valset
        for v in self.validators.iter() {
            if new_address_map.contains_key(&v.address) {
                return false;
            }
        }

        self.validators.extend(validators);

        return true;
    }

    pub fn remove_validators(&mut self, removed_validators: Integer) -> bool {
        if removed_validators.significant_bits() == 0 {
            return true;
        }

        if removed_validators.significant_bits() > self.validators.len() as u32 {
            return false;
        }

        // TODO: this is inneficient due to copies
        let mut tmp: Vec<Validator> = Vec::new();
        for (i, v) in self.validators.iter().enumerate() {
            if removed_validators.get_bit(i as u32) == false {
                tmp.push(*v);
            }
        }

        self.validators = tmp;
        return false;
    }
}
