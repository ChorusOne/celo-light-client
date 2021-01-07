mod relayer;
mod storage;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate celo;
use celo::*;
use relayer::*;

// TODO: https://github.com/rust-num/num-bigint/issues/172 (BIGINT bit)
// TODO: This file is temprory holder for those functions, clean this up afterwards
pub const EPOCH_SIZE: u64 = 17280;



#[tokio::main]
async fn main(){
    let relayer: Relayer = Relayer::new(
        "http://127.0.0.1:8545".to_string(),
    );

    let mut storage = storage::ExampleStorage::new("./local.db");
    //println!("{:?}", storage.put("test".to_string().as_bytes(), "test_val".to_string().as_bytes()));
    let mut state = State::new(EPOCH_SIZE, &mut storage);

    // insert and get, similar to std's BTreeMap
    //tree.insert("KEY1", "VAL1");
    //assert_eq!(tree.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));

    // -----

    let current_block_header: Header = relayer.get_block_header_by_number("latest".to_string()).await.unwrap();
    let current_epoch_number: u64 = get_epoch_number(current_block_header.number.to_u64().unwrap(), EPOCH_SIZE);
    let current_block_extra = IstanbulExtra::from_rlp(&current_block_header.extra).unwrap();

    // build up state from the genesis block to the latest

    for epoch_block_num in epoch_block_num_iter(0, current_epoch_number, EPOCH_SIZE) {
        let epoch_block_number_hex = format!("0x{:x}", epoch_block_num.clone());
        let header = relayer.get_block_header_by_number(epoch_block_number_hex.clone()).await;

        if header.is_ok() {
            println!("EPOCH BLOCK NUM: {:?} ({})", epoch_block_num.clone(), epoch_block_number_hex.clone());
            state.insert_epoch_header(&header.unwrap());
        } else {
            println!("EPOCH BLOCK NUM: {:?} ---- FAILED", epoch_block_num);
        }
    }

    //println!("STATE_HASH: {:?}", hex::encode(state.hash));
    //println!("STATE_NUMBER: {:?}", state.number);
    //println!("STATE_epoch: {:?}", state.epoch);
    //println!("STATE_VALIDATORS: {:?}", state.validators.iter().map(|v| hex::encode(v.address)).collect::<Vec<String>>());

    //let verify_result = verify_aggregated_seal(current_block_header.hash().unwrap(), state.validators, current_block_extra.aggregated_seal);
    //println!("LAST HEADER SEAL VERIFY: {:?}", verify_result);
}
