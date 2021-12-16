use crate::header::Header;

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Misbehaviour<T> {
    pub client_id: String,
    pub header_1: Header<T>,
    pub header_2: Header<T>,
}

