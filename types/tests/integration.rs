#![cfg(test)]
use celo_types::{client::*, consensus::*, istanbul::*, state::*, *};

use std::{fs::File, io::BufReader};

pub fn get_genesis() -> LightConsensusState {
    let blocks = read_headers();

    let genesis_header = blocks.first().unwrap();
    assert_eq!(genesis_header.number.as_u64(), 0, "no genesis block found");

    let ista_extra = extract_istanbul_extra(&genesis_header).unwrap();

    LightConsensusState {
        number: genesis_header.number.as_u64(),
        validators: ista_extra
            .added_validators
            .into_iter()
            .zip(ista_extra.added_validators_public_keys)
            .map(ValidatorData::from)
            .collect(),
        hash: genesis_header.root,
    }
}

pub fn get_header(idx: usize) -> Header {
    let blocks = read_headers();
    blocks.get(idx).unwrap().clone()
}

fn read_headers() -> Vec<Header> {
    let data_path = std::env::current_dir().unwrap().join("tests/baklava.json");

    let file = File::open(data_path).unwrap();
    let reader = BufReader::new(file);
    let mut blocks: Vec<Header> = serde_json::from_reader(reader).unwrap();
    blocks.sort_by_key(|b| b.number);
    blocks
}

#[test]
fn run_baklava() {
    let cfg = client::Config {
        epoch_size: 17280,
        allowed_clock_skew: 5,
        verify_epoch_headers: true,
        verify_non_epoch_headers: true,
        verify_header_timestamp: true,
    };
    let lcons = get_genesis();
    let mut heads = read_headers();
    heads.remove(0);

    let mut state = State::new(lcons, &cfg);
    for head in heads {
        let res = state.insert_header(&head, head.time.as_u64());
        assert!(
            res.is_ok(),
            format!("failed at header {}", head.number.as_u64())
        )
    }
}
