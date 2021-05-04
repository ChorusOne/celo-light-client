use crate::contract::types::ibc::{Channel, ConnectionEnd, Height, MerklePrefix};
use crate::contract::types::wasm::{
    ClientState, ConsensusState, CosmosClientState, CosmosConsensusState, Misbehaviour, WasmHeader,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum HandleMsg {
    InitializeState {
        consensus_state: ConsensusState,
        me: ClientState,
    },
    CheckHeaderAndUpdateState {
        header: WasmHeader,
        consensus_state: ConsensusState,
        me: ClientState,
    },
    CheckProposedHeaderAndUpdateState {
        header: WasmHeader,
        consensus_state: ConsensusState,
        me: ClientState,
    },
    VerifyUpgradeAndUpdateState {
        me: ClientState,
        new_client_state: ClientState,
        new_consensus_state: ConsensusState,
        client_upgrade_proof: String, // Go serializes []byte to base64 encoded string
        consensus_state_upgrade_proof: String, // Go serializes []byte to base64 encoded string
        last_height_consensus_state: ConsensusState,
    },
    CheckMisbehaviourAndUpdateState {
        me: ClientState,
        misbehaviour: Misbehaviour,
        consensus_state_1: ConsensusState,
        consensus_state_2: ConsensusState,
    },
    VerifyClientState {
        me: ClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        counterparty_client_identifier: String,
        proof: String, // Go serializes []byte to base64 encoded string
        counterparty_client_state: CosmosClientState,
        consensus_state: ConsensusState,
    },
    VerifyClientConsensusState {
        me: ClientState,
        height: Height,
        consensus_height: Height,
        commitment_prefix: MerklePrefix,
        counterparty_client_identifier: String,
        proof: String, // Go serializes []byte to base64 encoded string
        counterparty_consensus_state: CosmosConsensusState,
        consensus_state: ConsensusState,
    },
    VerifyConnectionState {
        me: ClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: String, // Go serializes []byte to base64 encoded string
        connection_id: String,
        connection_end: ConnectionEnd,
        consensus_state: ConsensusState,
    },
    VerifyChannelState {
        me: ClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: String, // Go serializes []byte to base64 encoded string
        port_id: String,
        channel_id: String,
        channel: Channel,
        consensus_state: ConsensusState,
    },
    VerifyPacketCommitment {
        me: ClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: String, // Go serializes []byte to base64 encoded string
        port_id: String,
        channel_id: String,
        current_timestamp: u64,
        delay_period: u64,
        sequence: u64,
        commitment_bytes: String, // Go serializes []byte to base64 encoded string
        consensus_state: ConsensusState,
    },
    VerifyPacketAcknowledgement {
        me: ClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: String, // Go serializes []byte to base64 encoded string
        port_id: String,
        channel_id: String,
        current_timestamp: u64,
        delay_period: u64,
        sequence: u64,
        acknowledgement: String, // Go serializes []byte to base64 encoded string
        consensus_state: ConsensusState,
    },
    VerifyPacketReceiptAbsence {
        me: ClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: String, // Go serializes []byte to base64 encoded string
        port_id: String,
        channel_id: String,
        current_timestamp: u64,
        delay_period: u64,
        sequence: u64,
        consensus_state: ConsensusState,
    },
    VerifyNextSequenceRecv {
        me: ClientState,
        height: Height,
        commitment_prefix: MerklePrefix,
        proof: String, // Go serializes []byte to base64 encoded string
        port_id: String,
        channel_id: String,
        current_timestamp: u64,
        delay_period: u64,
        next_sequence_recv: u64,
        consensus_state: ConsensusState,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct ClientStateCallResponse {
    pub me: ClientState,
    pub result: ClientStateCallResponseResult,
    pub new_client_state: ClientState,
    pub new_consensus_state: ConsensusState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct ClientStateCallResponseResult {
    pub is_valid: bool,
    pub err_msg: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct InitializeStateResult {
    pub result: ClientStateCallResponseResult,
    pub me: ClientState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct CheckHeaderAndUpdateStateResult {
    pub new_client_state: ClientState,
    pub new_consensus_state: ConsensusState,
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyUpgradeAndUpdateStateResult {
    pub result: ClientStateCallResponseResult,
    pub new_client_state: ClientState,
    pub new_consensus_state: ConsensusState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct CheckMisbehaviourAndUpdateStateResult {
    pub result: ClientStateCallResponseResult,
    pub new_client_state: ClientState,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyClientConsensusStateResult {
    pub result: ClientStateCallResponseResult,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct VerifyClientStateResult {
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
#[serde(rename_all = "lowercase")]
pub enum QueryMsg {
    ProcessedTime { height: Height },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct ProcessedTimeResponse {
    pub time: u64,
}

impl ClientStateCallResponseResult {
    pub fn success() -> Self {
       Self {
            is_valid: true,
            err_msg: "".to_owned(),
       }
    }
}
