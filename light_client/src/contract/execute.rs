use crate::contract::errors::{convert_celo, convert_rlp};
use crate::contract::msg;
use crate::contract::store::{
    get_consensus_state, set_consensus_meta, set_consensus_state, EMPTY_PREFIX, SUBJECT_PREFIX,
    SUBSTITUTE_PREFIX,
};
use crate::contract::util;
use crate::contract::{CeloClientState, CeloHeader};
use crate::contract::{WasmClientState, WasmConsensusState, WasmHeader, WasmMisbehaviour};

use celo_ibc::header::extract_header;
use celo_ibc::state::{extract_client, extract_consensus};
use celo_types::state::State as CeloState;
use celo_types::verify;
use cosmwasm_std::{
    Binary, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Timestamp, VerificationError,
};
use ibc_proto::ibc::core::client::v1::Height;

pub(crate) fn init_contract(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    consensus_state: WasmConsensusState,
    me: WasmClientState,
) -> StdResult<Response> {
    // Unmarshal Celo state
    let celo_consensus =
        extract_consensus(&consensus_state).map_err(|e| convert_rlp(e, "CeloConsensusState"))?;
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    // Verify initial state
    verify(&celo_consensus, &celo_client).map_err(convert_celo)?;
    if me.latest_height.revision_number != celo_client.chain_id
        || me.latest_height.revision_height != celo_consensus.number
    {
        return Err(StdError::VerificationErr {
            source: VerificationError::GenericErr,
        });
    }
    // Set metadata for initial consensus state
    set_consensus_meta(&env, deps.storage, EMPTY_PREFIX, &me.latest_height)?;
    // Update the state
    let latest_height = me.latest_height.clone();
    let response_data = &msg::InitializeStateResult {
        me,
        result: msg::ClientStateCallResponseResult::success(),
    };
    util::wrap_response_with_height(response_data, "init_block", &latest_height)
}

pub(crate) fn check_header_and_update_state(
    deps: DepsMut,
    env: Env,
    me: WasmClientState,
    consensus_state: WasmConsensusState,
    wasm_header: WasmHeader,
) -> StdResult<Response> {
    let current_timestamp: u64 = env.block.time.seconds();
    // Unmarshal celo states
    let celo_consensus =
        extract_consensus(&consensus_state).map_err(|e| convert_rlp(e, "CeloConsensusState"))?;
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    let celo_header = extract_header(&wasm_header).map_err(|e| convert_rlp(e, "CeloHeader"))?;
    // Ingest new header
    let mut state = CeloState::new(celo_consensus, &celo_client);
    if let Err(e) = state.insert_header(&celo_header, current_timestamp) {
        return Err(util::to_generic_err(format!(
            "Unable to ingest header. Error: {}",
            e
        )));
    }
    // Update the state
    let new_client_state = me;
    let new_consensus_state = WasmConsensusState::new(
        state.snapshot(),
        Timestamp::from_seconds(celo_header.time.as_u64()),
        celo_header.root,
    );
    // set metadata for this consensus state
    set_consensus_meta(&env, deps.storage, EMPTY_PREFIX, &wasm_header.height)?;
    let response_data = msg::CheckHeaderAndUpdateStateResult {
        new_client_state,
        new_consensus_state,
        result: msg::ClientStateCallResponseResult::success(),
    };
    util::wrap_response_with_height(response_data, "update_block", &wasm_header.height)
}

pub(crate) fn check_misbehaviour(
    _deps: DepsMut,
    _env: Env,
    me: WasmClientState,
    misbehaviour: WasmMisbehaviour,
    consensus_state1: WasmConsensusState,
    consensus_state2: WasmConsensusState,
) -> StdResult<Response> {
    // The header heights are expected to be the same
    if misbehaviour.header_1.height != misbehaviour.header_2.height {
        return Err(util::to_generic_err(format!(
            "Misbehaviour header heights differ, {:?} != {:?}",
            misbehaviour.header_1.height, misbehaviour.header_2.height
        )));
    }
    // If client is already frozen at earlier height than misbehaviour, return with error
    let already_frozen = me
        .frozen_height
        .clone()
        .map(|h| h <= misbehaviour.header_1.height)
        .unwrap_or_default();
    if already_frozen {
        return Err(util::to_generic_err(format!(
            "Client is already frozen at earlier height {:?} than misbehaviour height {:?}",
            &me.frozen_height, misbehaviour.header_1.height
        )));
    }
    // Unmarshal header
    let header_1: CeloHeader =
        extract_header(&misbehaviour.header_1).map_err(|e| convert_rlp(e, "CeloHeader 1"))?;
    let header_2: CeloHeader =
        extract_header(&misbehaviour.header_2).map_err(|e| convert_rlp(e, "CeloHeader 2"))?;
    // The header state root should differ
    if header_1.root == header_2.root {
        return Err(util::to_generic_err(
            "Header's state roots should differ, but are the same",
        ));
    }
    // Check the validity of the two conflicting headers against their respective
    // trusted consensus states
    util::check_misbehaviour_header(1, &me, &consensus_state1, &misbehaviour.header_1)?;
    util::check_misbehaviour_header(2, &me, &consensus_state2, &misbehaviour.header_2)?;
    // Store the new state
    let mut new_client_state = me;
    new_client_state.frozen_height = Some(misbehaviour.header_1.height.clone());
    let response_data = msg::CheckMisbehaviourAndUpdateStateResult {
        new_client_state,
        result: msg::ClientStateCallResponseResult::success(),
    };
    util::wrap_response_with_height(
        response_data,
        "verify_membership",
        &misbehaviour.header_1.height,
    )
}

pub(crate) fn verify_upgrade_and_update_state(
    deps: DepsMut,
    env: Env,
    me: WasmClientState,
    consensus_state: WasmConsensusState,
    new_client_state: WasmClientState,
    new_consensus_state: WasmConsensusState,
    _client_upgrade_proof: Binary,
    _consensus_state_upgrade_proof: Binary,
) -> StdResult<Response> {
    // Sanity check
    if new_client_state.latest_height <= me.latest_height {
        return Err(util::to_generic_err(format!(
            "upgraded client height {:?} must be at greater than current client height {:?}",
            new_client_state.latest_height, me.latest_height
        )));
    }
    // Unmarshal celo state
    let celo_consensus: CeloClientState =
        extract_consensus(&consensus_state).map_err(|e| convert_rlp(e, "CeloConsensusState"))?;
    let _celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    // Check consensus state expiration
    if util::is_expired(
        env.block.time,
        consensus_state.timestamp,
        celo_consensus.trusting_period,
    ) {
        return Err(util::to_generic_err("cannot upgrade an expired client"));
    }
    // Verify client proof
    // TODO!!!
    // set metadata for this consensus state
    set_consensus_meta(
        &env,
        deps.storage,
        EMPTY_PREFIX,
        &new_client_state.latest_height,
    )?;
    // Build up the response
    let response_data = msg::VerifyUpgradeAndUpdateStateResult {
        result: msg::ClientStateCallResponseResult::success(),
        new_client_state,
        new_consensus_state,
    };
    util::wrap_response(response_data, "verify_upgrade_and_update_state")
}

pub(crate) fn check_substitute_client_state(
    deps: DepsMut,
    env: Env,
    me: WasmClientState,
    substitute_client_state: WasmClientState,
    subject_consensus_state: WasmConsensusState,
    initial_height: Height,
) -> StdResult<Response> {
    if substitute_client_state.latest_height != initial_height {
        return Err(StdError::generic_err(format!(
           "substitute client revision number must equal initial height revision number ({:?} != {:?})",
           me.latest_height, initial_height
           )));
    }
    // Unmarshal celo state
    let _subject_consensus: CeloClientState = extract_consensus(&subject_consensus_state)
        .map_err(|e| convert_rlp(e, "CeloConsensusState"))?;
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    let substitute_client: CeloClientState =
        extract_client(&substitute_client_state).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    if me.frozen_height.is_some() && !celo_client.allow_update_after_misbehavior {
        return Err(util::to_generic_err("client is not allowed to be unfrozen"));
    }
    if celo_client != substitute_client {
        return Err(util::to_generic_err(
            "subject client state does not match substitute client state",
        ));
    }
    if util::is_expired(
        env.block.time,
        subject_consensus_state.timestamp,
        celo_client.trusting_period,
    ) && !celo_client.allow_update_after_expiry
    {
        return Err(util::to_generic_err(
            "client is not allowed to be unexpired",
        ));
    }
    let mut new_client_state = me.clone();
    new_client_state.frozen_height = None;
    // Copy consensus states and processed time from substitute to subject
    // starting from initial height and ending on the latest height (inclusive)
    let latest_height = substitute_client_state.latest_height.clone();
    for i in initial_height.revision_height..latest_height.revision_height + 1 {
        let height = Height {
            revision_height: i,
            revision_number: latest_height.revision_number,
        };
        let cs_bytes = get_consensus_state(deps.storage, SUBSTITUTE_PREFIX, &height);
        if cs_bytes.is_ok() {
            set_consensus_state(deps.storage, SUBJECT_PREFIX, &height, &cs_bytes.unwrap())?;
        }
        set_consensus_meta(&env, deps.storage, SUBJECT_PREFIX, &height)?;
    }
    new_client_state.latest_height = substitute_client_state.latest_height;
    let _latest_consensus_state_bytes =
        get_consensus_state(deps.storage, SUBJECT_PREFIX, &me.latest_height)?;
    let response_data = msg::CheckSubstituteAndUpdateStateResult {
        result: msg::ClientStateCallResponseResult::success(),
        new_client_state,
    };
    util::wrap_response(response_data, "check_substitute_and_update_state")
}

// zero_custom_fields returns a ClientState that is a copy of the current ClientState
// with all client customizable fields zeroed out
pub(crate) fn zero_custom_fields(
    _deps: DepsMut,
    _env: Env,
    me: WasmClientState,
) -> StdResult<Response> {
    let new_client_state = WasmClientState {
        data: me.data,
        ..Default::default()
    };

    // Build up the response
    util::wrap_response(
        &msg::ZeroCustomFieldsResult {
            me: new_client_state,
        },
        "zero_custom_fields",
    )
}
