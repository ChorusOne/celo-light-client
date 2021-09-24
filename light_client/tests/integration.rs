use std::str;

use cosmwasm_std::{from_binary, Coin, DepsMut, Env, MessageInfo, Response};
use cosmwasm_vm::testing;

use celo_ibc::{ClientState, ConsensusState, Height};
use celo_lightclient::contract;
use celo_lightclient::contract::msg;

// This line will test the output of cargo wasm
static WASM: &[u8] = include_bytes!(concat!(
    env!("CARGO_TARGET_DIR"),
    "/wasm32-unknown-unknown/release/celo_lightclient.wasm"
));

type DepsTest = cosmwasm_vm::Instance<testing::MockApi, testing::MockStorage, testing::MockQuerier>;

fn setup() -> (DepsTest, Env, MessageInfo) {
    let deps = testing::mock_instance(WASM, &[]);
    let env = testing::mock_env();
    let info = testing::mock_info("me", &[Coin::new(0, "gas")]);

    (deps, env, info)
}

fn setup_and_init() -> (DepsTest, Env, MessageInfo) {
    let (mut deps, env, info) = setup();
    let msg = msg::HandleMsg::InitializeState {
        consensus_state: ConsensusState::default(),
        me: ClientState::default(),
    };
    let resp: Response = testing::instantiate(&mut deps, env, info, msg).unwrap();

    let env = testing::mock_env();
    let info = testing::mock_info("me", &[Coin::new(0, "gas")]);
    (deps, env, info)
}

#[test]
fn test_init_contract_do_nothing() {
    let (mut deps, env, info) = setup();

    let msg = msg::HandleMsg::InitializeState {
        consensus_state: ConsensusState::default(),
        me: ClientState::default(),
    };

    let resp: Response = testing::instantiate(&mut deps, env, info, msg).unwrap();
    assert_eq!(resp.data, None);
    assert!(resp.events.is_empty());
    assert!(resp.messages.is_empty())
}

#[test]
fn test_init_contract() {
    let (mut deps, env, info) = setup_and_init();

    let msg = msg::HandleMsg::InitializeState {
        consensus_state: ConsensusState::default(),
        me: ClientState::default(),
    };

    let resp: Response = testing::execute(&mut deps, env, info, msg).unwrap();
    assert_eq!(
        resp.attributes.len(),
        2,
        "attributes ['action', 'last_consensus_state_height'] missing"
    );
    let action = resp.attributes.first().unwrap();
    assert_eq!(action.key, "action");
    assert_eq!(action.value, "init_block");
    let last = resp.attributes.last().unwrap();
    assert_eq!(last.key, "last_consensus_state_height");
    assert_eq!(last.value, "0");
    assert!(resp.data.is_some(), "there should be data");
    let bin_data = resp.data.unwrap();
    println!("json data {}", str::from_utf8(&bin_data).unwrap());
    let data: msg::InitializeStateResult = from_binary(&bin_data).unwrap();
    assert!(data.result.is_valid);
    assert_eq!(data.me.frozen_height, None);
    assert_eq!(data.me.latest_height.revision_height, 0);


}
