/// tese structures are meant to be exported by ibc-proto
/// however they are not yet available as github.com/cosmos/ibc-go repository
/// does not yet export the protobuf definitions, and, as a consequence,
/// ibc-proto does not yet provide such structures.
use ibc_proto::ibc::core::client::v1::Height;
use ibc_proto::ibc::core::commitment::v1::MerkleRoot;

/// WARNING: to be removed or replaced with
/// use ClientState = use ibc_proto::ibc::lightclients::wasm::v1::ClientState;
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ClientState {
    pub data: Vec<u8>,
    pub code_id: Vec<u8>,
    pub latest_height: Option<Height>,
}

/// WARNING: to be removed or replaced with
/// use ConsensusState = use ibc_proto::ibc::lightclients::wasm::v1::ConsensusState;
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ConsensusState {
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub root: Option<MerkleRoot>,
}

/// WARNING: to be removed or replaced with
/// use Header = use ibc_proto::ibc::lightclients::wasm::v1::Header;
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Header {
    pub data: Vec<u8>,
    pub height: Option<Height>,
}

/// WARNING: to be removed or replaced with
/// use Misbehaviour = use ibc_proto::ibc::lightclients::wasm::v1::Misbehaviour;
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Misbehaviour {
    pub client_id: String,
    pub header_1: Option<Header>,
    pub header_2: Option<Header>,
}
