#![cfg(test)]
use std::{fs::File, io::BufReader};

use celo_types::consensus::LightConsensusState;
use celo_types::istanbul::ValidatorData;
use celo_types::Header as CeloHeader;
use cosmwasm_std::{Coin, Env, MessageInfo, Response};
use cosmwasm_vm::testing;

use celo_ibc::state::{ClientState, ConsensusState};
use celo_lightclient::contract::msg;

type DepsTest = cosmwasm_vm::Instance<testing::MockApi, testing::MockStorage, testing::MockQuerier>;

pub fn setup(wasm: &[u8]) -> (DepsTest, Env, MessageInfo) {
    let deps = testing::mock_instance(wasm, &[]);
    let env = testing::mock_env();
    let info = testing::mock_info("me", &[Coin::new(0, "gas")]);

    (deps, env, info)
}

pub fn setup_and_init(wasm: &[u8]) -> (DepsTest, Env, MessageInfo) {
    let (mut deps, env, info) = setup(wasm);
    let msg = msg::HandleMsg::InitializeState {
        consensus_state: ConsensusState::default(),
        me: ClientState::default(),
    };
    let _resp: Response = testing::instantiate(&mut deps, env, info, msg).unwrap();

    let env = testing::mock_env();
    let info = testing::mock_info("me", &[Coin::new(0, "gas")]);
    (deps, env, info)
}

pub fn get_genesis() -> LightConsensusState {
    let blocks = read_headers();

    let genesis_header = blocks.first().unwrap();
    assert_eq!(genesis_header.number.as_u64(), 0, "no genesis block found");

    let ista_extra = celo_types::extract_istanbul_extra(&genesis_header).unwrap();

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

pub fn get_header(idx: usize) -> CeloHeader {
    let blocks = read_headers();
    blocks.get(idx).unwrap().clone()
}

fn read_headers() -> Vec<CeloHeader> {
    let data_path = std::env::current_dir().unwrap().join("tests/data.json");

    let file = File::open(data_path).unwrap();
    let reader = BufReader::new(file);
    let mut blocks: Vec<CeloHeader> = serde_json::from_reader(reader).unwrap();
    blocks.sort_by_key(|b| b.number);
    blocks
}
