use crate::types::header::{Header, Hash, Address};
use crate::types::istanbul::{IstanbulExtra, SerializedPublicKey};
use crate::errors::{Error, Kind};
use crate::bls::verify_aggregated_seal;
use crate::traits::Storage;
use crate::istanbul::{is_last_block_of_epoch, get_epoch_number};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use num_bigint::BigInt as Integer;
use num::cast::ToPrimitive;
use num_traits::Zero;

const LAST_ENTRY_HASH_KEY: &str = "last_entry_hash";

const ALLOWED_CLOCK_SKEW: u64 = 5;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Validator{
    #[serde(with = "crate::serialization::bytes::hexstring")]
    pub address: Address,

    #[serde(with = "crate::serialization::bytes::hexstring")]
    pub public_key: SerializedPublicKey,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct StateEntry {
    pub validators: Vec<Validator>, // set of authorized validators at this moment
    pub epoch: u64, // the number of blocks for each epoch
    pub number: u64, // block number where the snapshot was created
    pub hash: Hash, // block hash where the snapshot was created
}

impl StateEntry {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            epoch: 0,
            number: 0,
            hash: Hash::default()
        }
    }

    pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
        match serde_json::from_slice(&bytes) {
            Ok(entry) => Ok(entry),
            Err(e) => Err(Kind::JsonSerializationIssue.context(e).into())
        }
    }

    pub fn to_json(&self) -> Result<String, Error> {
        match serde_json::to_string(&self) {
            Ok(data) => Ok(data),
            Err(e) => Err(Kind::JsonSerializationIssue.context(e).into())
        }
    }
}

pub struct State {
    storage: Box<dyn Storage>,
    entry: StateEntry,
}

impl State {
    pub fn new(epoch: u64, storage: Box<dyn Storage>) -> Self {
        let mut entry = StateEntry::new();
        entry.epoch = epoch;
        
        State {
            storage,
            entry
        }
    }

    pub fn restore(&mut self) -> Result <u64, Error> {
        let last_hash = self.storage.get(LAST_ENTRY_HASH_KEY.as_bytes())?;
        let bytes = self.storage.get(&last_hash)?;

        self.entry = StateEntry::from_json(&bytes)?;

        Ok(get_epoch_number(self.entry.number, self.entry.epoch))
    }

    pub fn add_validators(&mut self, validators: Vec<Validator>) -> bool {
        let mut new_address_map: HashMap<Address, bool> = HashMap::new();
        
        for validator in validators.iter() {
            new_address_map.insert(validator.address, true);
        }

        // Verify that the validators to add is not already in the valset
        for v in self.entry.validators.iter() {
            if new_address_map.contains_key(&v.address) {
                return false;
            }
        }

        self.entry.validators.extend(validators);

        return true;
    }

    pub fn remove_validators(&mut self, removed_validators: &Integer) -> bool {
        if removed_validators.bits() == 0 {
            return true;
        }

        if removed_validators.bits() > self.entry.validators.len() as u64 {
            return false;
        }

        let filtered_validators: Vec<Validator> = self.entry.validators.iter()
            .enumerate()
            .filter(|(i, _)| removed_validators.bit(*i as u64) == false)
            .map(|(_, v)| v.to_owned())
            .collect();

        self.entry.validators = filtered_validators;

        return true;
    }

    pub fn verify_header(&self, header: &Header) -> Result<(), Error> {
        let header_hash = header.hash()?;
        let extra = IstanbulExtra::from_rlp(&header.extra)?;

        let curr_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(t) => t.as_secs(),
            Err(e) => return Err(Kind::Unknown.context(e).into()),
        };

	// don't waste time checking blocks from the future
	if header.time > curr_time + ALLOWED_CLOCK_SKEW {
            return Err(Kind::FutureBlock.into())
	}

        verify_aggregated_seal(
            header_hash,
            &self.entry.validators,
            extra.aggregated_seal
        )
    }

    pub fn insert_epoch_header(&mut self, header: &Header, verify: bool) -> Result<(), Error> {
        if !is_last_block_of_epoch(header.number.to_u64().unwrap(), self.entry.epoch) {
            return Err(Kind::InvalidChainInsertion.into());
        }

        // check if header is stored in the storage. If so, then update current validator set
        let key = header.hash()?;
        if self.storage.contains_key(&key)? {
            let entry = self.storage.get(&key)?;
            self.entry = StateEntry::from_json(&entry)?;
            return Ok(());
        }

        self.store_epoch_header(header, verify)
    }

    fn store_epoch_header(&mut self, header: &Header, verify: bool) -> Result<(), Error>{
        let header_hash = header.hash()?;
        let extra = IstanbulExtra::from_rlp(&header.extra)?;

        // genesis block is valid dead end
        if verify && !header.number.is_zero() {
            self.verify_header(&header)?
        }

        // convert istanbul validators into a Validator struct
        let mut validators: Vec<Validator> = Vec::new();
        if extra.added_validators.len() != extra.added_validators_public_keys.len() {
            return Err(Kind::InvalidValidatorSetDiff{msg: "error in combining addresses and public keys"}.into());
        }

        for i in 0..extra.added_validators.len() {
            validators.push(Validator{
                address: extra.added_validators[i].clone(),
                public_key: extra.added_validators_public_keys[i].clone(),
            })
        }

        // apply the header's changeset
        let result_remove = self.remove_validators(&extra.removed_validators);
        if !result_remove {
            return Err(Kind::InvalidValidatorSetDiff{msg: "error in removing the header's removed_validators"}.into());
        }

        let result_add = self.add_validators(validators);
        if !result_add {
            return Err(Kind::InvalidValidatorSetDiff{msg: "error in adding the header's added_validators"}.into());
        }

        let entry = StateEntry {
            validators: self.entry.validators.clone(),
            epoch: self.entry.epoch,
            number: self.entry.number + self.entry.epoch,
            hash: header_hash
        };

        let json_string = entry.to_json()?;

        // store header by it's number
        self.storage.put(
            entry.hash.as_ref(),
            json_string.as_ref(),
        )?;

        // update local state
        self.entry = entry;

        // update last entry marker
        self.storage.put(
            LAST_ENTRY_HASH_KEY.as_bytes(),
            self.entry.hash.as_ref()
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cmp, cmp::Ordering};
    use sha3::{Digest, Keccak256};
    use secp256k1::{rand::rngs::OsRng, Secp256k1, PublicKey, SecretKey};
    use crate::traits::{FromBytes, DefaultFrom};
    use crate::types::header::ADDRESS_LENGTH;

    macro_rules! string_vec {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    struct MockStorage{}

    impl Storage for MockStorage {
        fn put(&mut self, _key: &[u8], _value: &[u8]) -> Result<Option<Vec<u8>>, Error> {
            Ok(None)
        }
    
        fn get(&self, _key: &[u8]) -> Result<Vec<u8>, Error> {
            Ok(Vec::new())
        }
    
        fn contains_key(&self, _key: &[u8]) -> Result<bool, Error> {
            Ok(false)
        }
    }

    struct TestValidatorSet {
        validators: Vec<String>,
        validator_set_diffs: Vec<ValidatorSetDiff>,
        results: Vec<String>,
    }

    struct ValidatorSetDiff {
        added_validators: Vec<String>,
        removed_validators: Vec<String>
    }

    struct AccountPool {
        pub accounts: HashMap<String, (SecretKey, PublicKey)>,
    }

    impl AccountPool {
        fn new() -> Self {
            Self {
                accounts: HashMap::new(),
            }
        }

        fn address(&mut self, account: String) -> Address {
            if account == "" {
                return Address::default();
            }

            if !self.accounts.contains_key(&account) {
                self.accounts.insert(account.clone(), generate_key());
            }

            pubkey_to_address(self.accounts.get(&account).unwrap().1)
        }
    }

    #[test]
    fn test_add_remove() {
        let storage = Box::new(MockStorage{});
        let mut state = State::new(123, storage);
        let mut result = state.add_validators(vec![
            Validator{
                address: bytes_to_address(&vec![0x3 as u8]),
                public_key: SerializedPublicKey::default(),
            }
        ]);

        assert_eq!(result, true);

        result = state.add_validators(vec![
            Validator{
                address: bytes_to_address(&vec![0x2 as u8]),
                public_key: SerializedPublicKey::default(),
            },
            Validator{
                address: bytes_to_address(&vec![0x1 as u8]),
                public_key: SerializedPublicKey::default(),
            }
        ]);

        assert_eq!(result, true);
        assert_eq!(state.entry.validators.len(), 3);

        // verify ordering
        let current_addresses: Vec<Address> = state.entry.validators.iter().map(|val| val.address).collect();
        let expecected_addresses: Vec<Address> = vec![
            bytes_to_address(&vec![0x3 as u8]),
            bytes_to_address(&vec![0x2 as u8]),
            bytes_to_address(&vec![0x1 as u8]),
        ];
        assert_eq!(current_addresses, expecected_addresses);

        // remove first validator
        result = state.remove_validators(&Integer::from(1));
        assert_eq!(result, true);
        assert_eq!(state.entry.validators.len(), 2);

        // remove second validator
        result = state.remove_validators(&Integer::from(2));
        assert_eq!(result, true);
        assert_eq!(state.entry.validators.len(), 1);

        // remove third validator
        result = state.remove_validators(&Integer::from(1));
        assert_eq!(result, true);
        assert_eq!(state.entry.validators.len(), 0);
    }

    #[test]
    fn applies_validator_set_changes() {
        let tests = vec![
            // Single validator, empty val set diff
            TestValidatorSet {
                validators: string_vec!["A"],
                validator_set_diffs: vec![
                    ValidatorSetDiff{
                        added_validators: Vec::new(),
                        removed_validators: Vec::new(),
                    }
                ],
                results: string_vec!["A"],
            },
            // Single validator, add two new validators
            TestValidatorSet {
                validators: string_vec!["A"],
                validator_set_diffs: vec![
                    ValidatorSetDiff{
                        added_validators: string_vec!["B", "C"],
                        removed_validators: Vec::new(),
                    }
                ],
                results: string_vec!["A", "B", "C"],
            },
            // Two validator, remove two validators
            TestValidatorSet {
                validators: string_vec!["A", "B"],
                validator_set_diffs: vec![
                    ValidatorSetDiff{
                        added_validators: Vec::new(),
                        removed_validators: string_vec!["A", "B"],
                    }
                ],
                results: string_vec![],
            },
            // Three validator, add two validators and remove two validators
            TestValidatorSet {
                validators: string_vec!["A", "B", "C"],
                validator_set_diffs: vec![
                    ValidatorSetDiff{
                        added_validators: string_vec!["D", "E"],
                        removed_validators: string_vec!["B", "C"],
                    }
                ],
                results: string_vec!["A", "D", "E"],
            },
            // Three validator, add two validators and remove two validators.  Second header will add 1 validators and remove 2 validators.
            TestValidatorSet {
                validators: string_vec!["A", "B", "C"],
                validator_set_diffs: vec![
                    ValidatorSetDiff{
                        added_validators: string_vec!["D", "E"],
                        removed_validators: string_vec!["B", "C"],
                    },
                    ValidatorSetDiff{
                        added_validators: string_vec!["F"],
                        removed_validators: string_vec!["A", "D"],
                    }
                ],
                results: string_vec!["F", "E"],
            },
        ];

        for test in tests {
            let storage = Box::new(MockStorage{});
            let mut accounts = AccountPool::new();
            let mut state = State::new(123, storage);

            let validators = convert_val_names_to_validators(&mut accounts, test.validators);
            state.add_validators(validators.clone());

            for diff in test.validator_set_diffs {
                let added_validators = convert_val_names_to_validators(&mut accounts, diff.added_validators);
                let removed_validators = convert_val_names_to_removed_validators(&mut accounts, &state.entry.validators, diff.removed_validators);

                state.remove_validators(&removed_validators);
                state.add_validators(added_validators);
            }

            let results = convert_val_names_to_validators(&mut accounts, test.results);
            assert_eq!(compare(state.entry.validators, results), Ordering::Equal);
        }
    }

    pub fn compare(a: Vec<Validator>, b :Vec<Validator>) -> cmp::Ordering {
        let mut sorted_a = a.clone();
        let mut sorted_b = b.clone();

        sorted_a.sort_by(|a, b| b.address.cmp(&a.address));
        sorted_b.sort_by(|a, b| b.address.cmp(&a.address));

        sorted_a.iter()
            .zip(sorted_b)
            .map(|(x, y)| x.address.cmp(&y.address))
            .find(|&ord| ord != cmp::Ordering::Equal)
            .unwrap_or(a.len().cmp(&b.len()))
    }

    fn pubkey_to_address(p: PublicKey) -> Address {
       let pub_bytes = p.serialize_uncompressed();
       let digest = &Keccak256::digest(&pub_bytes[1..])[12..];

       Address::from_bytes(digest).unwrap().to_owned()
    }

    fn bytes_to_address(bytes: &[u8]) -> Address {
        let mut v = vec![0x0; ADDRESS_LENGTH - bytes.len()];
        v.extend_from_slice(bytes);

        Address::from_bytes(&v).unwrap().to_owned()
    }

    fn generate_key() -> (SecretKey, PublicKey) {
        let mut rng = OsRng::new().expect("OsRng");
        let secp = Secp256k1::new();

        secp.generate_keypair(&mut rng)
    }

    fn convert_val_names_to_validators(accounts: &mut AccountPool, val_names: Vec<String>) -> Vec<Validator> {
        val_names.iter().map(|name| Validator{
            address: accounts.address(name.to_string()),
            public_key: SerializedPublicKey::default(),
        }).collect()
    }

    fn convert_val_names_to_removed_validators(accounts: &mut AccountPool, old_validators: &[Validator], val_names: Vec<String>) -> Integer {
        let mut bitmap = Integer::from(0);
        for v in val_names {
            for j in 0..old_validators.len() {
                if &accounts.address(v.to_string()) == &old_validators.get(j).unwrap().address {
                    bitmap.set_bit(j as u64, true);
                }
            }
        }

        bitmap
    }
}
