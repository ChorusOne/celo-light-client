#![allow(dead_code)]

pub mod msg;
pub use msg::*;
mod cosmos;
mod errors;
mod execute;
mod identifier;
mod query;
mod serialization;
mod store;
mod util;

use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo};
use cosmwasm_std::{QueryResponse, Response, StdError, StdResult};

pub(crate) type WasmClientState = celo_ibc::state::ClientState;
pub(crate) type CeloClientState = celo_types::client::LightClientState;

pub(crate) type WasmConsensusState = celo_ibc::state::ConsensusState;
pub(crate) type CeloConsensusState = celo_types::consensus::LightConsensusState;

pub(crate) type WasmHeader = celo_ibc::header::Header;
pub(crate) type CeloHeader = celo_types::header::Header;

pub(crate) type WasmMisbehaviour = celo_ibc::misbehaviour::Misbehaviour;

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: HandleMsg,
) -> StdResult<Response> {
    // The 10-wasm Init method is split into two calls, where the second (via handle())
    // call expects ClientState included in the return.
    //
    // Therefore it's better to execute whole logic in the second call.
    Ok(Response::default())
}
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<Response, StdError> {
    match msg {
        HandleMsg::InitializeState {
            consensus_state,
            me,
        } => execute::init_contract(deps, env, info, consensus_state, me),
        HandleMsg::CheckHeaderAndUpdateState {
            header,
            consensus_state,
            me,
        } => execute::check_header_and_update_state(deps, env, me, consensus_state, header),
        HandleMsg::CheckMisbehaviourAndUpdateState {
            me,
            misbehaviour,
            consensus_state_1,
            consensus_state_2,
        } => execute::check_misbehaviour(
            deps,
            env,
            me,
            misbehaviour,
            consensus_state_1,
            consensus_state_2,
        ),
        HandleMsg::VerifyUpgradeAndUpdateState {
            me,
            consensus_state,
            new_client_state,
            new_consensus_state,
            client_upgrade_proof,
            consensus_state_upgrade_proof,
        } => execute::verify_upgrade_and_update_state(
            deps,
            env,
            me,
            consensus_state,
            new_client_state,
            new_consensus_state,
            client_upgrade_proof,
            consensus_state_upgrade_proof,
        ),
        HandleMsg::CheckSubstituteAndUpdateState {
            me,
            substitute_client_state,
            subject_consensus_state,
            initial_height,
        } => execute::check_substitute_client_state(
            deps,
            env,
            me,
            substitute_client_state,
            subject_consensus_state,
            initial_height,
        ),
        HandleMsg::ZeroCustomFields { me } => execute::zero_custom_fields(deps, env, me),
    }
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::VerifyClientState {
            me,
            height,
            commitment_prefix,
            counterparty_client_identifier,
            proof,
            counterparty_client_state,
            consensus_state,
        } => query::verify_client_state(
            deps,
            env,
            me,
            height,
            commitment_prefix,
            counterparty_client_identifier,
            proof,
            counterparty_client_state,
            consensus_state,
        ),
        QueryMsg::VerifyClientConsensusState {
            me,
            height,
            consensus_height,
            commitment_prefix,
            counterparty_client_identifier,
            proof,
            counterparty_consensus_state,
            consensus_state,
        } => query::verify_client_consensus_state(
            deps,
            env,
            me,
            height,
            consensus_height,
            commitment_prefix,
            counterparty_client_identifier,
            proof,
            counterparty_consensus_state,
            consensus_state,
        ),
        QueryMsg::VerifyConnectionState {
            me,
            height,
            commitment_prefix,
            proof,
            connection_id,
            connection_end,
            consensus_state,
        } => query::verify_connection_state(
            deps,
            env,
            me,
            height,
            commitment_prefix,
            proof,
            connection_id,
            connection_end,
            consensus_state,
        ),
        QueryMsg::VerifyChannelState {
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            channel,
            consensus_state,
        } => query::verify_channel_state(
            deps,
            env,
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            channel,
            consensus_state,
        ),
        QueryMsg::VerifyPacketCommitment {
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            delay_time_period,
            delay_block_period,
            sequence,
            commitment_bytes,
            consensus_state,
        } => query::verify_packet_commitment(
            deps,
            env,
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            delay_time_period,
            delay_block_period,
            sequence,
            commitment_bytes,
            consensus_state,
        ),
        QueryMsg::VerifyPacketAcknowledgement {
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            delay_time_period,
            delay_block_period,
            sequence,
            acknowledgement,
            consensus_state,
        } => query::verify_packet_acknowledgment(
            deps,
            env,
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            delay_time_period,
            delay_block_period,
            sequence,
            acknowledgement,
            consensus_state,
        ),
        QueryMsg::VerifyPacketReceiptAbsence {
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            delay_time_period,
            delay_block_period,
            sequence,
            consensus_state,
        } => query::verify_packet_receipt_absence(
            deps,
            env,
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            delay_time_period,
            delay_block_period,
            sequence,
            consensus_state,
        ),
        QueryMsg::VerifyNextSequenceRecv {
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            delay_time_period,
            delay_block_period,
            next_sequence_recv,
            consensus_state,
        } => query::verify_next_sequence_recv(
            deps,
            env,
            me,
            height,
            commitment_prefix,
            proof,
            port_id,
            channel_id,
            delay_time_period,
            delay_block_period,
            next_sequence_recv,
            consensus_state,
        ),
        QueryMsg::Status {
            me,
            consensus_state,
        } => query::status(deps, env, me, consensus_state),
    }
}
