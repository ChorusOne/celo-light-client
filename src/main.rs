mod types;
mod serialization;
mod relayer;
mod istanbul;
mod state;
mod crypto;
mod traits;
mod macros;
mod errors;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate rlp;
extern crate rug;
extern crate sha3;
extern crate secp256k1;
extern crate bls_crypto;
extern crate algebra;
extern crate anomaly;
extern crate thiserror;



// TODO: https://github.com/rust-num/num-bigint/issues/172 (BIGINT bit)
use types::header::Header;
use types::istanbul::IstanbulExtra;
use relayer::Relayer;
use istanbul::*;
use state::*;
use crypto::bls::*;

fn firstn(first_epoch: u64, max_epoch: u64) -> impl std::iter::Iterator<Item = u64> {
    let mut current_epoch = first_epoch;
    std::iter::from_fn(move || {
        let result;
        if current_epoch < max_epoch {
            result = Some(get_epoch_last_block_number(current_epoch, EPOCH_SIZE));
            current_epoch += 1
        } else {
            result = None
        }
        result
    })
}

#[tokio::main]
async fn main(){
    let relayer: Relayer = Relayer::new("http://127.0.0.1:8545".to_string());

    let current_block_header: Header = relayer.get_block_header_by_number("latest".to_string()).await.unwrap();
    let current_epoch_number: u64 = get_epoch_number(current_block_header.number.to_u64().unwrap(), EPOCH_SIZE);
    let current_block_extra = IstanbulExtra::from_rlp(&current_block_header.extra).unwrap();

    // build up state from the genesis block to the latest
    let mut state = State::new();

    for (i, epoch_block_num) in firstn(0, current_epoch_number).enumerate() {
        //println!("IS_LAST_BLOCK_OF_EPOCH: {}", is_last_block_of_epoch(epoch_block_num, EPOCH_SIZE));
        let epoch_block_number_hex = format!("0x{:x}", epoch_block_num.clone());
        let header = relayer.get_block_header_by_number(epoch_block_number_hex.clone()).await;
        //println!("{}", serde_json::to_string_pretty(&header.number).unwrap());

        if header.is_ok() {
            println!("EPOCH BLOCK NUM: {:?} ({})", epoch_block_num.clone(), epoch_block_number_hex.clone());
            let header = header.unwrap();

            let extra = IstanbulExtra::from_rlp(&header.extra).unwrap();

            let mut validators: Vec<Validator> = Vec::new();
            for i in 0..extra.added_validators.len() {
                validators.push(Validator{
                    address: extra.added_validators.get(i).unwrap().clone(),
                    public_key: extra.added_validators_public_keys.get(i).unwrap().clone(),
                })
            }

            assert_eq!(extra.added_validators.len(), validators.len());

            let result_remove = state.remove_validators(extra.removed_validators.clone());
            let result_add = state.add_validators(validators);

            if !result_remove || !result_add {
                println!("-----------");
                println!("EPOCH BLOCK NUM: {:?} ({})", epoch_block_num.clone(), epoch_block_number_hex.clone());
                return;
            }

            state.epoch = i as u64; // TODO
            state.number += EPOCH_SIZE; //TODO
            state.hash = header.hash();
        } else {
            println!("EPOCH BLOCK NUM: {:?} ---- FAILED", epoch_block_num);
        }
    }

    println!("STATE_HASH: {:?}", hex::encode(state.hash));
    println!("STATE_NUMBER: {:?}", state.number);
    println!("STATE_epoch: {:?}", state.epoch);
    println!("STATE_VALIDATORS: {:?}", state.validators.iter().map(|v| hex::encode(v.address)).collect::<Vec<String>>());

    let verify_result = verify_aggregated_seal(current_block_header.hash(), state.validators, current_block_extra.aggregated_seal);
    println!("LAST HEADER SEAL VERIFY: {:?}", verify_result);
}
