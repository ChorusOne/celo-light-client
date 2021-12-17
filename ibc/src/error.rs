use thiserror::Error;

/// All error kinds related to the light client.
#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("in struct {struct_name} missing field {field_name}")]
    MissingField {
        struct_name: String,
        field_name: String,
    },

    #[error(transparent)]
    RlpDecodeError(#[from] rlp::DecoderError),

    #[error(transparent)]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}
