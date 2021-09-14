use anomaly::{BoxError, Context};
use thiserror::Error;

/// The main error type verification methods will return.
/// See [`Kind`] for the different kind of errors.
pub type Error = anomaly::Error<Kind>;

/// All error kinds related to the light client.
#[derive(Clone, Debug, Error)]
pub enum Kind {
    #[error("invalid data length while converting slice to fixed-size array type ({current} != {expected}")]
    InvalidDataLength { current: usize, expected: usize },

    #[error("rlp decode error")]
    RlpDecodeError,

    #[error("header verification failed: {msg}")]
    HeaderVerificationError { msg: &'static str },

    #[error("invalid validator set diff: {msg}")]
    InvalidValidatorSetDiff { msg: &'static str },

    #[cfg(feature = "web3_support")]
    #[error("missing field {field}")]
    MissingField{field: String},

}

impl Kind {
    /// Add additional context.
    pub fn context(self, source: impl Into<BoxError>) -> Context<Kind> {
        Context::new(self, Some(source.into()))
    }
}
