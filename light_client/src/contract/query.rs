use crate::contract::cosmos;
use crate::contract::errors::{convert_celo, convert_rlp};
use crate::contract::identifier;
use crate::contract::msg;
use crate::contract::store;
use crate::contract::{CeloClientState, WasmClientState, WasmConsensusState};

use celo_ibc::state::{extract_client, extract_consensus};
use celo_types::proof::Proof;
use celo_types::verify;

use ibc_proto::ibc::core::channel::v1::Channel;
use ibc_proto::ibc::core::client::v1::Height;
use ibc_proto::ibc::core::commitment::v1::MerklePrefix;
use ibc_proto::ibc::core::connection::v1::ConnectionEnd;

use cosmwasm_std::{to_binary, Binary, Deps, Env, QueryResponse, StdError, StdResult, Timestamp};
use ethereum_types::U256;

pub(crate) fn verify_client_state(
    _deps: Deps,
    _env: Env,
    me: WasmClientState,
    _height: Height,
    prefix: MerklePrefix,
    counterparty_client_identifier: String,
    proof: Binary,
    counterparty_client_state: cosmos::ClientState,
    proving_client_state: WasmConsensusState,
) -> StdResult<QueryResponse> {
    // Unmarshal Celo state
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    // Unmarshal proof
    let proof: Proof = cosmwasm_std::from_binary(&proof)?;
    let key = identifier::client_commitment_key(
        celo_client.commitment_map_position,
        prefix,
        &counterparty_client_identifier,
    )?;
    let value = identifier::hash_committed_value(&counterparty_client_state);
    verify::verify(
        &proof,
        proving_client_state.root(),
        celo_client.counterparty_address,
        key,
        Some(value),
    )
    .map_err(convert_celo)?;

    // Build up the response
    to_binary(&msg::VerifyClientStateResult {
        result: msg::ClientStateCallResponseResult::success(),
    })
}

pub(crate) fn verify_client_consensus_state(
    _deps: Deps,
    _env: Env,
    me: WasmClientState,
    _height: Height,
    consensus_height: Height,
    prefix: MerklePrefix,
    counterparty_client_identifier: String,
    proof: Binary,
    counterparty_consensus_state: cosmos::ConsensusState,
    proving_consensus_state: WasmConsensusState,
) -> StdResult<QueryResponse> {
    // Unmarshal Celo state
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    // Unmarshal proof
    let proof: Proof = cosmwasm_std::from_binary(&proof)?;
    let key = identifier::consensus_commitment_key(
        celo_client.commitment_map_position,
        prefix,
        &counterparty_client_identifier,
        &consensus_height,
    )?;
    let value = identifier::hash_committed_value(&counterparty_consensus_state);
    verify::verify(
        &proof,
        proving_consensus_state.root(),
        celo_client.counterparty_address,
        key,
        Some(value),
    )
    .map_err(convert_celo)?;
    // Build up the response
    to_binary(&msg::VerifyClientConsensusStateResult {
        result: msg::ClientStateCallResponseResult::success(),
    })
}

pub(crate) fn verify_connection_state(
    _deps: Deps,
    _env: Env,
    me: WasmClientState,
    _height: Height,
    prefix: MerklePrefix,
    proof: Binary,
    connection_id: String,
    connection_end: ConnectionEnd,
    proving_consensus_state: WasmConsensusState,
) -> StdResult<QueryResponse> {
    // Unmarshal Celo state
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    // Unmarshal proof
    let proof: Proof = cosmwasm_std::from_binary(&proof)?;
    let key = identifier::connection_commitment_key(
        celo_client.commitment_map_position,
        prefix,
        &connection_id,
    )?;
    let value = identifier::hash_committed_value(&connection_end);
    verify::verify(
        &proof,
        proving_consensus_state.root(),
        celo_client.counterparty_address,
        key,
        Some(value),
    )
    .map_err(convert_celo)?;
    // Build up the response
    to_binary(&msg::VerifyConnectionStateResult {
        result: msg::ClientStateCallResponseResult::success(),
    })
}

pub(crate) fn verify_channel_state(
    _deps: Deps,
    _env: Env,
    me: WasmClientState,
    _height: Height,
    prefix: MerklePrefix,
    proof: Binary,
    port_id: String,
    channel_id: String,
    channel: Channel,
    proving_consensus_state: WasmConsensusState,
) -> StdResult<QueryResponse> {
    // Unmarshal Celo state
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    // Unmarshal proof
    let proof: Proof = cosmwasm_std::from_binary(&proof)?;
    let key = identifier::channel_commitment_key(
        celo_client.commitment_map_position,
        prefix,
        &port_id,
        &channel_id,
    )?;
    let value = identifier::hash_committed_value(&channel);
    verify::verify(
        &proof,
        proving_consensus_state.root(),
        celo_client.counterparty_address,
        key,
        Some(value),
    )
    .map_err(convert_celo)?;
    // Build up the response
    to_binary(&msg::VerifyChannelStateResult {
        result: msg::ClientStateCallResponseResult::success(),
    })
}

pub(crate) fn verify_packet_commitment(
    deps: Deps,
    env: Env,
    me: WasmClientState,
    height: Height,
    prefix: MerklePrefix,
    proof: Binary,
    port_id: String,
    channel_id: String,
    delay_time_period: u64,
    delay_block_period: u64,
    sequence: u64,
    commitment_bytes: Binary,
    proving_consensus_state: WasmConsensusState,
) -> StdResult<QueryResponse> {
    // Unmarshal Celo state
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    verify_delay_period_passed(
        deps,
        height,
        env.block.height,
        env.block.time,
        delay_time_period,
        delay_block_period,
    )?;
    // Unmarshal proof
    let proof: Proof = cosmwasm_std::from_binary(&proof)?;
    let key = identifier::packet_key(
        celo_client.commitment_map_position,
        prefix,
        &port_id,
        &channel_id,
        sequence,
    )?;
    let value = U256::from(commitment_bytes.as_slice());
    verify::verify(
        &proof,
        proving_consensus_state.root(),
        celo_client.counterparty_address,
        key,
        Some(value),
    )
    .map_err(convert_celo)?;
    // Build up the response
    to_binary(&msg::VerifyPacketCommitmentResult {
        result: msg::ClientStateCallResponseResult::success(),
    })
}

pub(crate) fn verify_packet_acknowledgment(
    deps: Deps,
    env: Env,
    me: WasmClientState,
    height: Height,
    prefix: MerklePrefix,
    proof: Binary,
    port_id: String,
    channel_id: String,
    delay_time_period: u64,
    delay_block_period: u64,
    sequence: u64,
    acknowledgement: Binary,
    proving_consensus_state: WasmConsensusState,
) -> StdResult<QueryResponse> {
    // Unmarshal Celo state
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    verify_delay_period_passed(
        deps,
        height,
        env.block.height,
        env.block.time,
        delay_time_period,
        delay_block_period,
    )?;
    // Unmarshal proof
    let proof: Proof = cosmwasm_std::from_binary(&proof)?;
    let key = identifier::packet_key(
        celo_client.commitment_map_position,
        prefix,
        &port_id,
        &channel_id,
        sequence,
    )?;
    let value = identifier::packet_commit_ack(&acknowledgement);
    verify::verify(
        &proof,
        proving_consensus_state.root(),
        celo_client.counterparty_address,
        key,
        Some(value),
    )
    .map_err(convert_celo)?;
    // Build up the response
    to_binary(&msg::VerifyPacketAcknowledgementResult {
        result: msg::ClientStateCallResponseResult::success(),
    })
}

pub(crate) fn verify_packet_receipt_absence(
    deps: Deps,
    env: Env,
    me: WasmClientState,
    height: Height,
    prefix: MerklePrefix,
    proof: Binary,
    port_id: String,
    channel_id: String,
    delay_time_period: u64,
    delay_block_period: u64,
    sequence: u64,
    proving_consensus_state: WasmConsensusState,
) -> StdResult<QueryResponse> {
    // Unmarshal Celo state
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    verify_delay_period_passed(
        deps,
        height,
        env.block.height,
        env.block.time,
        delay_time_period,
        delay_block_period,
    )?;
    // Unmarshal proof
    let proof: Proof = cosmwasm_std::from_binary(&proof)?;
    let key = identifier::packet_key(
        celo_client.commitment_map_position,
        prefix,
        &port_id,
        &channel_id,
        sequence,
    )?;
    verify::verify(
        &proof,
        proving_consensus_state.root(),
        celo_client.counterparty_address,
        key,
        None,
    )
    .map_err(convert_celo)?;
    // Build up the response
    to_binary(&msg::VerifyPacketReceiptAbsenceResult {
        result: msg::ClientStateCallResponseResult::success(),
    })
}

pub(crate) fn verify_next_sequence_recv(
    deps: Deps,
    env: Env,
    me: WasmClientState,
    height: Height,
    prefix: MerklePrefix,
    proof: Binary,
    port_id: String,
    channel_id: String,
    delay_time_period: u64,
    delay_block_period: u64,
    next_sequence_recv: u64,
    proving_consensus_state: WasmConsensusState,
) -> StdResult<QueryResponse> {
    // Unmarshal Celo state
    let celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    verify_delay_period_passed(
        deps,
        height,
        env.block.height,
        env.block.time,
        delay_time_period,
        delay_block_period,
    )?;
    // Unmarshal proof
    let proof: Proof = cosmwasm_std::from_binary(&proof)?;
    let key = identifier::packet_key(
        celo_client.next_sequence_rx_map_position,
        prefix,
        &port_id,
        &channel_id,
        next_sequence_recv,
    )?;
    let value = U256::from(next_sequence_recv);
    verify::verify(
        &proof,
        proving_consensus_state.root(),
        celo_client.counterparty_address,
        key,
        Some(value),
    )
    .map_err(convert_celo)?;
    // Build up the response
    to_binary(&msg::VerifyPacketAcknowledgementResult {
        result: msg::ClientStateCallResponseResult::success(),
    })
}

pub(crate) fn status(
    _deps: Deps,
    _env: Env,
    me: WasmClientState,
    consensus_state: WasmConsensusState,
) -> StdResult<QueryResponse> {
    // Unmarshal Celo state
    let _celo_client: CeloClientState =
        extract_client(&me).map_err(|e| convert_rlp(e, "CeloClientState"))?;
    let _celo_consensus: CeloClientState =
        extract_consensus(&consensus_state).map_err(|e| convert_rlp(e, "CeloConsensusState"))?;

    let mut status = msg::Status::Active;

    if me.frozen_height.is_some() {
        status = msg::Status::Frozen;
    }

    // Build up the response
    to_binary(&msg::StatusResult { status })
}

fn verify_delay_period_passed(
    deps: Deps,
    proof_height: Height,
    current_height: u64,
    current_timestamp: Timestamp,
    delay_time_period: u64,
    delay_block_period: u64,
) -> StdResult<()> {
    let processed_time =
        store::get_processed_time(deps.storage, store::EMPTY_PREFIX, &proof_height)?;
    let valid_time = processed_time.plus_seconds(delay_time_period);

    if current_timestamp < valid_time {
        return Err(StdError::generic_err(format!(
            "cannot verify packet until time: {}, current time: {}",
            valid_time, current_timestamp
        )));
    }

    let processed_height: Height =
        store::get_processed_height(deps.storage, store::EMPTY_PREFIX, &proof_height)?;
    let valid_height = Height {
        revision_number: processed_height.revision_number,
        revision_height: processed_height.revision_height + delay_block_period,
    };

    let current_height = store::get_self_height(current_height);
    if current_height < valid_height {
        return Err(StdError::generic_err(format!(
            "cannot verify packet until height: {:?}, current height: {:?}",
            valid_height, current_height
        )));
    }
    Ok(())
}
