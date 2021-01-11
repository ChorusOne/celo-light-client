use anomaly::{BoxError, Context};
use thiserror::Error;

/// The main error type verification methods will return.
/// See [`Kind`] for the different kind of errors.
pub type Error = anomaly::Error<Kind>;

/// All error kinds related to the light client.
#[derive(Clone, Debug, Error)]
pub enum Kind {
    /// Invalid data length while converting slice to fixed-size array type
    #[error("invalid data length while converting slice to fixed-size array type ({current} != {expected}")]
    InvalidDataLength { current: usize, expected: usize },

    #[error("rlp decode error")]
    RlpDecodeError,

    #[error("invalid validator set diff: {msg}")]
    InvalidValidatorSetDiff { msg: &'static str },

    #[error("attempted to insert invalid data to chain")]
    InvalidChainInsertion,

    #[error("aggregated seal does not aggregate enough seals, num_seals: {current}, minimum quorum size: {expected}")]
    MissingSeals{ current: usize, expected: usize },

    #[error("BLS verify error")]
    BlsVerifyError,

    #[error("BLS invalid signature")]
    BlsInvalidSignature,

    #[error("BLS invalid public key")]
    BlsInvalidPublicKey,

    #[error("JSON serialization issue")]
    JsonSerializationIssue,

    #[error("encountered a block with time set in the future")]
    FutureBlock,

    #[error("unkown error occurred")]
    Unknown,
}

impl Kind {
    /// Add additional context.
    pub fn context(self, source: impl Into<BoxError>) -> Context<Kind> {
        Context::new(self, Some(source.into()))
    }
}
