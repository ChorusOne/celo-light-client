use crate::header::Header;

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Misbehaviour {
    pub client_id: String,
    pub header_1: Header,
    pub header_2: Header,
}

