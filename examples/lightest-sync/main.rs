mod relayer;
mod storage;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

extern crate celo;
use celo::*;
use relayer::*;

use clap::{App, Arg};

// TODO: https://github.com/rust-num/num-bigint/issues/172 (BIGINT bit)

extern crate log;
use log::{info, error};

#[tokio::main]
async fn main(){
    env_logger::init();

    // setup CLI
    let matches = App::new("lightest-sync-example")
        .version("1.0")
        .author("Mateusz Kaczanowski <mateusz@chorus.one>")
        .about("Demonstrates lightest-sync library usage for Celo blockchain")
        .arg(
                Arg::with_name("fast")
                .short("f")
                .long("fast")
                .takes_value(false)
                .help("Skips the seal verification for the epoch headers (to build up current validator set faster)")
        )
        .arg(
                Arg::with_name("epoch-size")
                .short("e")
                .long("epoch-size")
                .takes_value(true)
                .default_value("17280")
                .help("The epoch-size of Celo blockchain")
        )
        .arg(
                Arg::with_name("db")
                .short("d")
                .long("db")
                .takes_value(true)
                .default_value("./local.db")
                .help("The path to local database")
        )
        .arg(
                Arg::with_name("addr")
                .short("a")
                .long("addr")
                .takes_value(true)
                .default_value("http://127.0.0.1:8545")
                .help("Removes the local database")
        )
        .get_matches();

    let validate_all_headers = match matches.occurrences_of("fast") {
        1 => false,
        _ => true,
    };

    let epoch_size = value_t!(matches.value_of("epoch-size"), u64).unwrap();
    let db_path = matches.value_of("db").unwrap();
    let addr = matches.value_of("addr").unwrap();

    // setup relayer
    info!("Setting up relayer");
    let relayer: Relayer = Relayer::new(addr.to_string());

    // setup state container
    info!("Setting up storage");
    let mut storage = storage::ExampleStorage::new(db_path);
    let mut state = State::new(epoch_size, &mut storage);

    info!("Restoring previous state from DB (if applicable)");
    let first_epoch: u64 = state.restore().unwrap_or_default();

    info!("Fetching latest block header from: {}", addr);
    let current_block_header: Header = relayer.get_block_header_by_number("latest").await.unwrap();
    let current_epoch_number: u64 = get_epoch_number(current_block_header.number.to_u64().unwrap(), epoch_size);

    info!(
        "Syncing epoch headers from {} to epoch num: {} (last header num: {}, epoch size: {})",
        first_epoch, current_epoch_number, current_block_header.number, epoch_size
    );

    // build up state from the genesis block to the latest
    for (i, epoch_block_num) in epoch_block_num_iter(first_epoch, current_epoch_number, epoch_size).enumerate() {
        let epoch = 1 + i as u64 + first_epoch;
        let epoch_block_number_hex = format!("0x{:x}", epoch_block_num);
        let header = relayer.get_block_header_by_number(&epoch_block_number_hex).await;

        if header.is_ok() {
            match state.insert_epoch_header(&header.unwrap(), validate_all_headers) {
                Ok(_) => info!("[{}/{}] Inserted epoch header: {}", epoch, current_epoch_number, epoch_block_number_hex),
                Err(e) => error!("Failed to insert epoch header {}: {}", epoch_block_number_hex, e)
            }
        } else {
            error!("Failed to fetch block header num: {}", epoch_block_number_hex);
        }
    }

    match state.verify_header(&current_block_header) {
        Ok(_) => info!("Succesfully validated latest header against local state: {}", current_block_header.number),
        Err(e) => error!("Failed to validate latest header against local state: {}", e)
    }
}
