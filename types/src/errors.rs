use anomaly::{BoxError, Context};
use thiserror::Error;
use ethereum_types::U256;

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

    #[cfg(feature = "web3-support")]
    #[error("missing field {field}")]
    MissingField { field: String },

    #[error("BLS verify error")]
    BlsVerifyError,

    #[error("BLS invalid signature")]
    BlsInvalidSignature,

    #[error("BLS invalid public key")]
    BlsInvalidPublicKey,

    #[error("aggregated seal does not aggregate enough seals, num_seals: {current}, minimum quorum size: {expected}")]
    MissingSeals { current: usize, expected: usize },

    #[error("Storage key is not present in the proof")]
    StorageProofKeyNotPresent,

    #[error("Storage key is not matching, current: {current} vs expected: {expected}")]
    StorageProofKeyNotMatching{current: U256, expected: U256},

    #[error("Storage value is not matching, current: {current} vs expected: {expected}")]
    StorageProofValueNotMatching{current: U256, expected: U256},

    #[error("proof verification error: {error}")]
    ProofVerification{error: String},
}

impl Kind {
    /// Add additional context.
    pub fn context(self, source: impl Into<BoxError>) -> Context<Kind> {
        Context::new(self, Some(source.into()))
    }
}
