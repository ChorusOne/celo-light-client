mod common;

use cosmwasm_std::{from_binary, from_slice, Response};
use cosmwasm_vm::testing;

use celo_ibc::{ClientState, ConsensusState, Header, Height, MerkleRoot};
use celo_lightclient::contract::msg;
use celo_types::client::LightClientState;

// This line will test the output of cargo wasm
static WASM: &[u8] = include_bytes!(concat!(
    env!("CARGO_TARGET_DIR"),
    "/wasm32-unknown-unknown/release/celo_lightclient.wasm"
));

#[test]
fn test_init_contract_do_nothing() {
    let (mut deps, env, info) = common::setup(WASM);

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
    let (mut deps, env, info) = common::setup_and_init(WASM);

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
    let data: msg::InitializeStateResult = from_binary(&bin_data).unwrap();
    assert!(data.result.is_valid);
    assert_eq!(data.me.frozen_height, None);
    assert_eq!(data.me.latest_height.revision_height, 0);
}

#[test]
fn test_zero_custom_fields_contract_call() {
    let (mut deps, env, info) = common::setup(WASM);
    let msg = msg::HandleMsg::ZeroCustomFields{
    me: ClientState::default(),
    };

    let resp: Response = testing::execute(&mut deps, env, info, msg).unwrap();

    let action = resp.attributes.first().unwrap();
    assert_eq!(action.key, "action");
    assert_eq!(action.value, "zero_custom_fields");
    let last = resp.attributes.last().unwrap();
    assert!(resp.data.is_some(), "there should be data");
    let bin_data = resp.data.unwrap();
    let data: msg::ZeroCustomFieldsResult = from_slice(&bin_data).unwrap();
    assert_eq!(data.me.frozen_height, None);
    assert_eq!(data.me.latest_height.revision_height, 0);
}

#[test]
fn check_header_and_update_state() {
    let (mut deps, env, info) = common::setup_and_init(WASM);
    let light_cons = common::get_genesis();
    let celo_header = common::get_header(1);
    let light_client = LightClientState {
        verify_non_epoch_headers: true,
        epoch_size: 17280,
        ..Default::default()
    };
    let latest_h = Height {
        revision_number: 0,
        revision_height: celo_header.number.as_u64(),
    };

    let wasm_header = Header::new(&celo_header, latest_h.clone());
    let cons = ConsensusState::new(
        &light_cons,
        String::new(),
        celo_header.time.as_u64(),
        MerkleRoot::from(celo_header.root),
    );
    let client = ClientState::new(&light_client, String::new(), latest_h, Vec::new());

    let msg = msg::HandleMsg::CheckHeaderAndUpdateState {
        header: wasm_header,
        consensus_state: cons,
        me: client,
    };

    let resp: Response = testing::execute(&mut deps, env, info, msg).unwrap();

    let action = resp.attributes.first().unwrap();
    assert_eq!(action.key, "action");
    assert_eq!(action.value, "update_block");
    let last = resp.attributes.last().unwrap();
    assert_eq!(last.key, "last_consensus_state_height");
    assert_eq!(last.value, "1");
}
