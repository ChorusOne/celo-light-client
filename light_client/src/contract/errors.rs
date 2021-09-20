use anomaly::{BoxError, Context};
use thiserror::Error;
use std::convert::From;

/// The main error type verification methods will return.
/// See [`Kind`] for the different kind of errors.
pub type Error = anomaly::Error<Kind>;

/// All error kinds related to the light client.
#[derive(Clone, Debug, Error)]
pub enum Kind {
    #[error("error from celo-type: {error}")]
    CeloTypeError { error: String },

    #[error("error from wasm-ibc: {error}")]
    WasmIbcError { error: String },

    #[error("attempted to insert invalid data to chain")]
    InvalidChainInsertion,

    #[error("aggregated seal does not aggregate enough seals, num_seals: {current}, minimum quorum size: {expected}")]
    MissingSeals { current: usize, expected: usize },

    #[error("BLS verify error")]
    BlsVerifyError,

    #[error("BLS invalid signature")]
    BlsInvalidSignature,

    #[error("BLS invalid public key")]
    BlsInvalidPublicKey,

    #[error("unkown error occurred")]
    Unknown,
}

impl From<celo_types::errors::Error> for Kind {
    fn from(e : celo_types::errors::Error) -> Self {
        Kind::CeloTypeError{error : format!("{}", e)}
    }
}

impl Kind {
    /// Add additional context.
    pub fn context(self, source: impl Into<BoxError>) -> Context<Kind> {
        Context::new(self, Some(source.into()))
    }
}
