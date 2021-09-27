use crate::{Error, Height, Kind};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Header {
    pub data: String,
    pub height: Height,
}
impl Header {
    pub fn new(ch: &celo_types::Header, height: Height) -> Self {
        let r = rlp::encode(ch);
        Self {
            data: base64::encode(r),
            height,
        }
    }
}
impl Default for Header {
    fn default() -> Self {
        let lc = celo_types::Header::default();
        Header::new(&lc, Height::default())
    }
}

pub fn extract_header(h: &Header) -> Result<celo_types::Header, Error> {
    let v: Vec<u8> = base64::decode(&h.data).map_err(|e| {
        let k: Kind = e.into();
        let e: Error = k.into();
        e
    })?;
    rlp::decode(&v).map_err(|e| {
        let k: Kind = e.into();
        k.into()
    })
}
