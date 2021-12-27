use crate::contract::{WasmMisbehaviour, WasmHeader, WasmClientState, WasmConsensusState };
use crate::contract::cosmos;

use cosmwasm_std::Binary;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ibc_proto::ibc::core::client::v1::Height;
use ibc_proto::ibc::core::commitment::v1::MerklePrefix;
use ibc_proto::ibc::core::channel::v1::Channel;
use ibc_proto::ibc::core::connection::v1::ConnectionEnd;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum HandleMsg {
    InitializeState {
        consensus_state: WasmConsensusState,
        me: WasmClientState,
    },
    CheckHeaderAndUpdateState {
        header: WasmHeader,
        consensus_state: WasmConsensusState,
        me: WasmClientState,
    },
    VerifyUpgradeAndUpdateState {
        me: WasmClientState,
        consensus_state: WasmConsensusState,
        new_client_state: WasmClientState,
        new_consensus_state: WasmConsensusState,
        client_upgrade_proof: Binary,
        consensus_state_upgrade_proof: Binary,
    },
    CheckMisbehaviourAndUpdateState {
        me: WasmClientState,
        misbehaviour: WasmMisbehaviour,
        consensus_state_1: WasmConsensusState,
        consensus_state_2: WasmConsensusState,
    },
    CheckSubstituteAndUpdateState {
        me: WasmClientState,
        substitute_client_state: WasmClientState,
        subject_consensus_state: WasmConsensusState,
        initial_height: Height,
    },
    ZeroCustomFields {
        me: WasmClientState,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum QueryMsg {
    VerifyClientState {
        me: WasmClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        counterparty_client_identifier: String,
        proof: Binary,
        counterparty_client_state: cosmos::ClientState,
        consensus_state: WasmConsensusState,
    },
    VerifyClientConsensusState {
        me: WasmClientState,
        height: Height,
        consensus_height: Height,
        commitment_prefix: MerklePrefix,
        counterparty_client_identifier: String,
        proof: Binary,
        counterparty_consensus_state: cosmos::ConsensusState,
        consensus_state: WasmConsensusState,
    },
    VerifyConnectionState {
        me: WasmClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: Binary,
        connection_id: String,
        connection_end: ConnectionEnd,
        consensus_state: WasmConsensusState,
    },
    VerifyChannelState {
        me: WasmClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: Binary,
        port_id: String,
        channel_id: String,
        channel: Channel,
        consensus_state: WasmConsensusState,
    },
    VerifyPacketCommitment {
        me: WasmClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: Binary,
        port_id: String,
        channel_id: String,
        delay_time_period: u64,
        delay_block_period: u64,
        sequence: u64,
        commitment_bytes: Binary,
        consensus_state: WasmConsensusState,
    },
    VerifyPacketAcknowledgement {
        me: WasmClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: Binary,
        port_id: String,
        channel_id: String,
        delay_time_period: u64,
        delay_block_period: u64,
        sequence: u64,
        acknowledgement: Binary,
        consensus_state: WasmConsensusState,
    },
    VerifyPacketReceiptAbsence {
        me: WasmClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: Binary,
        port_id: String,
        channel_id: String,
        delay_time_period: u64,
        delay_block_period: u64,
        sequence: u64,
        consensus_state: WasmConsensusState,
    },
    VerifyNextSequenceRecv {
        me: WasmClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: Binary,
        port_id: String,
        channel_id: String,
        delay_time_period: u64,
        delay_block_period: u64,
        next_sequence_recv: u64,
        consensus_state: WasmConsensusState,
    },
    Status {
        me: WasmClientState,
        consensus_state: WasmConsensusState,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct ClientStateCallResponseResult {
    pub is_valid: bool,
    pub err_msg: String,
}
impl ClientStateCallResponseResult {
    pub fn success() -> Self {
        Self {
            is_valid: true,
            err_msg: "".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct InitializeStateResult {
    pub result: ClientStateCallResponseResult,
    pub me: WasmClientState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct CheckHeaderAndUpdateStateResult {
    pub new_client_state: WasmClientState,
    pub new_consensus_state: WasmConsensusState,
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct CheckMisbehaviourAndUpdateStateResult {
    pub result: ClientStateCallResponseResult,
    pub new_client_state: WasmClientState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyUpgradeAndUpdateStateResult {
    pub result: ClientStateCallResponseResult,
    pub new_client_state: WasmClientState,
    pub new_consensus_state: WasmConsensusState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct CheckSubstituteAndUpdateStateResult {
    pub result: ClientStateCallResponseResult,
    pub new_client_state: WasmClientState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct ZeroCustomFieldsResult {
    pub me: WasmClientState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyClientStateResult {
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyClientConsensusStateResult {
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyConnectionStateResult {
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyChannelStateResult {
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyPacketCommitmentResult {
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyPacketAcknowledgementResult {
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyPacketReceiptAbsenceResult {
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct StatusResult {
    pub status: Status,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub enum Status {
    Active,
    Frozen,
    Exipred,
    Unknown
}

/*
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct ProcessedTimeResponse {
    pub time: u64,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct ClientStateCallResponse {
    pub me: ClientState,
    pub result: ClientStateCallResponseResult,
    pub new_client_state: ClientState,
    pub new_consensus_state: WasmConsensusState,
}

*/
