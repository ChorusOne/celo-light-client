use celo_types::errors::Error as CeloError;
use cosmwasm_std::{StdError, VerificationError};

pub(crate) fn convert_celo(c_error: CeloError) -> StdError {
    match c_error {
        CeloError::IstanbulDataLength { current, expected } => StdError::InvalidDataSize {
            expected: expected as u64,
            actual: current as u64,
        },
        CeloError::MissingField(field) => StdError::NotFound { kind: field },
        CeloError::RlpDecodeError(kind, e) => StdError::ParseErr {
            target_type: kind,
            msg: e.to_string(),
        },
        CeloError::HeaderVerificationError(msg) => StdError::GenericErr {
            msg: format!("CeloError::HeaderVerificationError, {}", msg),
        },
        CeloError::InvalidValidatorSetDiff(msg) => StdError::GenericErr {
            msg: format!("CeloError::InvalidValidatorSetDiff, {}", msg),
        },
        CeloError::BlsVerifyError => StdError::GenericErr {
            msg: String::from("CeloError::BlsVerifyError"),
        },
        CeloError::BlsInvalidSignature => StdError::VerificationErr {
            source: VerificationError::InvalidSignatureFormat,
        },
        CeloError::BlsInvalidPublicKey => StdError::VerificationErr {
            source: VerificationError::InvalidPubkeyFormat,
        },
        CeloError::MissingSeals { current, expected } => StdError::GenericErr {
            msg: format!(
                "CeloError::MissingSeals, expected: {}, current: {}",
                expected, current
            ),
        },
        CeloError::StorageProofKeyNotPresent => StdError::GenericErr {
            msg: String::from("CeloError::StorageProofKeyNotPresent"),
        },
        CeloError::StorageProofKeyNotMatching { current, expected } => StdError::GenericErr {
            msg: String::from("CeloError::StorageProofKeyNotMatching"),
        },
        CeloError::StorageProofValueNotMatching { current, expected } => StdError::GenericErr {
            msg: String::from("CeloError::StorageProofValueNotMatching"),
        },
        CeloError::ProofVerification(msg) => StdError::GenericErr {
            msg: String::from("CeloError::ProofVerification"),
        },
    }
}

pub(crate) fn convert_rlp(e: rlp::DecoderError, target: &str) -> StdError {
    StdError::ParseErr {
        target_type: target.to_string(),
        msg: e.to_string(),
    }
}
