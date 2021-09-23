mod relayer;
use relayer::Relayer;

use celo_types::client::Config;
use celo_types::consensus::LightConsensusState;
use celo_types::state::State;
use celo_types::{get_epoch_last_block_number, get_epoch_number, Header};

use clap::{value_t, App, Arg};
use log::{error, info};
use std::time::{SystemTime, UNIX_EPOCH};
use web3::types::{BlockNumber, U64};

fn main() {
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
                .default_value("http://localhost:8545")
                .help("Removes the local database")
        )
        .get_matches();

    let validate_all_headers = match matches.occurrences_of("fast") {
        1 => false,
        _ => true,
    };

    let first_epoch = 0;
    let epoch_size = value_t!(matches.value_of("epoch-size"), u64).unwrap();
    let addr = matches.value_of("addr").unwrap();

    // setup relayer
    info!("Setting up relayer");
    let relayer: Relayer = Relayer::new(addr.to_string());

    // setup state container
    info!("Setting up storage");
    let state_config = Config {
        epoch_size,
        allowed_clock_skew: 5,

        verify_epoch_headers: validate_all_headers,
        verify_non_epoch_headers: validate_all_headers,
        verify_header_timestamp: true,
    };
    let snapshot = LightConsensusState::default();
    let mut state = State::new(snapshot, &state_config);

    info!("Fetching latest block header from: {}", addr);
    let current_block_header: Header = relayer.get_block_header_by_number(BlockNumber::Latest);
    let current_epoch_number: u64 =
        get_epoch_number(current_block_header.number.as_u64(), epoch_size);

    info!(
        "Syncing epoch headers from {} to epoch num: {} (last header num: {}, epoch size: {})",
        first_epoch, current_epoch_number, current_block_header.number, epoch_size
    );

    // build up state from the genesis block to the latest
    for epoch in first_epoch..current_epoch_number {
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let epoch_block_num = get_epoch_last_block_number(epoch, epoch_size);
        let header =
            relayer.get_block_header_by_number(BlockNumber::Number(U64::from(epoch_block_num)));

        match state.insert_header(&header, current_timestamp) {
            Ok(_) => info!(
                "[{}/{}] Inserted epoch header: {}",
                epoch + 1,
                current_epoch_number,
                epoch_block_num
            ),
            Err(e) => error!("Failed to insert epoch header {}: {}", epoch_block_num, e),
        }
    }
}
