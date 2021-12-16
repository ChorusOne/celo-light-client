use anomaly::{BoxError, Context};
use thiserror::Error;

/// The main error type verification methods will return.
/// See [`Kind`] for the different kind of errors.
pub type Error = anomaly::Error<Kind>;

/// All error kinds related to the light client.
#[derive(Clone, Debug, Error)]
pub enum Kind {
    #[error("in struct {struct_name} missing field {field_name}")]
    MissingField {
        struct_name: String,
        field_name: String,
    },

    #[error("rlp decode error: {rlp_error}")]
    RlpDecodeError { rlp_error: rlp::DecoderError },

    #[error("TryFromSlice error: {from_slice_error}")]
    TryFromSliceError {
        from_slice_error: std::array::TryFromSliceError,
    },
}

impl Kind {
    /// Add additional context.
    pub fn context(self, source: impl Into<BoxError>) -> Context<Kind> {
        Context::new(self, Some(source.into()))
    }
}

impl From<rlp::DecoderError> for Kind {
    fn from(rlp_error: rlp::DecoderError) -> Self {
        Kind::RlpDecodeError { rlp_error }
    }
}
