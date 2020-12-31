mod types;
mod serialization;
mod relayer;
mod istanbul;
mod state;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate rlp;
extern crate rug;
extern crate sha3;


// TODO: https://github.com/rust-num/num-bigint/issues/172 (BIGINT bit)
// use std::fs;
use types::header::Header;
use types::istanbul::IstanbulExtra;
use relayer::Relayer;
use istanbul::*;
use state::*;

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

    // build up state from the genesis block to the latest
    let mut state = State::new();

    for (i, epoch_block_num) in firstn(0, current_epoch_number).enumerate() {
        let epoch_block_number_hex = format!("0x{:x}", epoch_block_num);
        let header = relayer.get_block_header_by_number(epoch_block_number_hex).await;
        //println!("{}", serde_json::to_string_pretty(&header.number).unwrap());

        if header.is_ok() {
            println!("EPOCH BLOCK NUM: {:?} ", epoch_block_num);
            let header = header.unwrap();

            let extra = IstanbulExtra::from_rlp(&header.extra).unwrap();

            let mut validators: Vec<Validator> = Vec::new();
            for i in 0..extra.added_validators.len() {
                validators.push(Validator{
                    address: extra.added_validators.get(i).unwrap().clone(),
                    public_key: extra.added_validators_public_keys.get(i).unwrap().clone(),
                })
            }

            state.remove_validators(extra.removed_validators);
            state.add_validators(validators);
            state.epoch = i as u64; // TODO
            state.number += EPOCH_SIZE; //TODO
            state.hash = header.hash();

            //println!("EXTRA ADDED: {:?}", IstanbulExtra::from_rlp(&header.extra).unwrap().added_validators);
            //println!("EXTRA REMOVED: {:?}", IstanbulExtra::from_rlp(&header.extra).unwrap().removed_validators);
        } else {
            println!("EPOCH BLOCK NUM: {:?} ---- FAILED", epoch_block_num);
        }
    }

    println!("STATE_VALIDATORS: {:?}", state.validators);

    //let contents = fs::read_to_string("/tmp/t")
        //.expect("Something went wrong reading the file");
    //let header: Header = serde_json::from_str(contents.as_str()).unwrap();
    //println!("{}", serde_json::to_string_pretty(&header).unwrap());
}
