use crate::types::header::{Hash, Address};
use crate::types::istanbul::SerializedPublicKey;
use std::collections::HashMap;
use rug::Integer;

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
        return true;
    }
}

pub fn min_quorum_size(validators: &[Validator]) -> u64 {
    return ((2.0*(validators.len() as f64) / 3.0) as f64).ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha3::{Digest, Keccak256};
    use secp256k1::rand::rngs::OsRng;
    use secp256k1::{PublicKey, Secp256k1, SecretKey};
    use std::{cmp, cmp::Ordering};
    use crate::traits::default::DefaultFrom;

    macro_rules! string_vec {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
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
        pub accounts: HashMap<String, (secp256k1::SecretKey, secp256k1::PublicKey)>,
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

            pubkey_to_address(self.accounts.get(&account).unwrap().1) // TODO: see if this is okay
        }
    }

    fn convert_val_names_to_validators(accounts: &mut AccountPool, val_names: Vec<String>) -> Vec<Validator> {
        val_names.iter().map(|name| Validator{
            address: accounts.address(name.to_string()),
            public_key: SerializedPublicKey::default(),
        }).collect()
    }

    fn pubkey_to_address(p: secp256k1::PublicKey) -> Address {
       let pub_bytes = p.serialize_uncompressed();
       let digest = &Keccak256::digest(&pub_bytes[1..])[12..];
       let mut address = Address::default();

       address.copy_from_slice(digest);
       address
    }

    fn bytes_to_address(bytes: &[u8]) -> Address {
        let mut address = [0; 20];

        for (i, byte) in bytes.iter().enumerate() {
            address[address.len()-i-1] = *byte;
        }
        address
    }

    fn generate_key() -> (SecretKey, PublicKey) {
        let mut rng = OsRng::new().expect("OsRng");
        let secp = Secp256k1::new();

        secp.generate_keypair(&mut rng)
    }

    fn convert_val_names_to_removed_validators(accounts: &mut AccountPool, old_validators: &[Validator], val_names: Vec<String>) -> Integer {
        let mut bitmap = Integer::from(0);
        for v in val_names {
            for j in 0..old_validators.len() {
                if &accounts.address(v.to_string()) == &old_validators.get(j).unwrap().address {
                    bitmap.set_bit(j as u32, true);
                }
            }
        }

        bitmap
    }

    #[test]
    fn test_add_remove() {
        let mut state = State::new();
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
        assert_eq!(state.validators.len(), 3);

        // verify ordering
        let current_addresses: Vec<Address> = state.validators.iter().map(|val| val.address).collect();
        let expecected_addresses: Vec<Address> = vec![
            bytes_to_address(&vec![0x3 as u8]),
            bytes_to_address(&vec![0x2 as u8]),
            bytes_to_address(&vec![0x1 as u8]),
        ];
        assert_eq!(current_addresses, expecected_addresses);

        // remove first validator
        result = state.remove_validators(Integer::from(1));
        assert_eq!(result, true);
        assert_eq!(state.validators.len(), 2);

        // remove second validator
        result = state.remove_validators(Integer::from(2));
        assert_eq!(result, true);
        assert_eq!(state.validators.len(), 1);

        // remove third validator
        result = state.remove_validators(Integer::from(1));
        assert_eq!(result, true);
        assert_eq!(state.validators.len(), 0);
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
            let mut accounts = AccountPool::new();
            let mut state = State::new();

            let validators = convert_val_names_to_validators(&mut accounts, test.validators);
            state.add_validators(validators.clone());

            for diff in test.validator_set_diffs {
                let added_validators = convert_val_names_to_validators(&mut accounts, diff.added_validators);
                let removed_validators = convert_val_names_to_removed_validators(&mut accounts, &state.validators, diff.removed_validators);

                state.remove_validators(removed_validators);
                state.add_validators(added_validators);
            }

            let results = convert_val_names_to_validators(&mut accounts, test.results);
            assert_eq!(compare(state.validators, results), Ordering::Equal);
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
}
