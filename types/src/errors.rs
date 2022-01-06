use ethereum_types::U256;
use thiserror::Error;

/// All error kinds related to the light client.
#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("Istanbul extra field too short,{expected} > {current}")]
    IstanbulDataLength { expected: usize, current: usize },

    #[error("RlpError while decoding {0}, err: {1}")]
    RlpDecodeError(String, rlp::DecoderError),

    #[error("header verification failed: {0}")]
    HeaderVerificationError(String),

    #[error("invalid validator set diff: {0}")]
    InvalidValidatorSetDiff(String),

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
    StorageProofKeyNotMatching { current: U256, expected: U256 },

    #[error("Storage value is not matching, current: {current} vs expected: {expected}")]
    StorageProofValueNotMatching { current: U256, expected: U256 },

    #[error("proof verification error: {0}")]
    ProofVerification(String),
}

#[cfg(feature = "web3-support")]
pub mod web3 {
    #[derive(Clone, Default, Debug)]
    pub struct MissingFieldErr(pub String);
    impl std::fmt::Display for MissingFieldErr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MissingField {}", self.0)
        }
    }
}
