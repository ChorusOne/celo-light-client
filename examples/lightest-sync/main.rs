mod relayer;
mod storage;

#[macro_use]
extern crate serde_derive;

extern crate celo;
use celo::*;
use relayer::*;

// TODO: https://github.com/rust-num/num-bigint/issues/172 (BIGINT bit)
// TODO: This file is temprory holder for those functions, clean this up afterwards
pub const EPOCH_SIZE: u64 = 17280;

extern crate log;
use log::{info, error};

#[tokio::main]
async fn main(){
    env_logger::init();

    // setup relayer
    info!("Setting up relayer");
    let addr = "http://127.0.0.1:8545".to_string();
    let relayer: Relayer = Relayer::new(addr.clone());

    // setup state container
    info!("Setting up storage");
    let mut storage = storage::ExampleStorage::new("./local.db");
    let mut state = State::new(EPOCH_SIZE, &mut storage);

    info!("Restoring previous state from DB (if applicable)");
    let first_epoch: u64 = state.restore().unwrap_or_default();

    info!("Fetching latest block header from: {}", addr);
    let current_block_header: Header = relayer.get_block_header_by_number("latest").await.unwrap();
    let current_epoch_number: u64 = get_epoch_number(current_block_header.number.to_u64().unwrap(), EPOCH_SIZE);

    info!(
        "Syncing epoch headers from {} to epoch num: {} (last header num: {}, epoch size: {})",
        first_epoch, current_epoch_number, current_block_header.number, EPOCH_SIZE
    );

    // build up state from the genesis block to the latest
    for (i, epoch_block_num) in epoch_block_num_iter(first_epoch, current_epoch_number, EPOCH_SIZE).enumerate() {
        let epoch = i as u64 + first_epoch;
        let epoch_block_number_hex = format!("0x{:x}", epoch_block_num);
        let header = relayer.get_block_header_by_number(&epoch_block_number_hex).await;

        if header.is_ok() {
            match state.insert_epoch_header(&header.unwrap()) {
                Ok(_) => info!("[{}/{}] Inserted epoch header: {}", epoch, current_epoch_number, epoch_block_number_hex),
                Err(e) => error!("Failed to insert epoch header {}: {}", epoch_block_number_hex, e)
            }
        } else {
            error!("Failed to fetch block header num: {}", epoch_block_number_hex);
        }
    }
}
