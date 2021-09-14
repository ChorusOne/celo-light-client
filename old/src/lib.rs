//! This library contains a portion of Celo Blockchain logic and structures, that is primarily
//! used by Celo Light client contract deployed on Cosmos Network.
//!
//! In particular, the library provides the LightestSync method to quickly, securely and cheaply
//! synchronize IBFT consensus state with Celo chain.

mod types;
mod serialization;
mod state;
mod istanbul;
mod bls;
mod traits;
mod macros;
mod errors;

#[macro_use]
extern crate serde;

extern crate rlp;
extern crate num_bigint;
extern crate sha3;
extern crate bls_crypto;
extern crate algebra;
extern crate anomaly;
extern crate thiserror;

pub use types::{
    header::Header,
    header::Address,
    header::Hash,
    istanbul::SerializedPublicKey,
    istanbul::IstanbulExtra,
    state::Validator,
    state::Snapshot,
    state::Config
};
pub use istanbul::{
    get_epoch_number,
    get_epoch_first_block_number,
    get_epoch_last_block_number,
};
pub use state::State;
pub use errors::{Error, Kind};
pub use traits::{
    FromBytes,
    DefaultFrom,
    ToRlp,
    FromRlp
};
pub use bls::verify_aggregated_seal;

#[cfg(feature = "wasm-contract")]
pub mod contract;

#[cfg(all(feature = "wasm-contract", target_arch = "wasm32"))]
cosmwasm_std::create_entry_points!(contract);
