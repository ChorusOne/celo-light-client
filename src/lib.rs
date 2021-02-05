pub mod types;
pub mod serialization;
pub mod istanbul;
pub mod state;
pub mod bls;
pub mod traits;
pub mod macros;
pub mod errors;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate rlp;
extern crate num_bigint;
extern crate sha3;
extern crate secp256k1;
extern crate bls_crypto;
extern crate algebra;
extern crate anomaly;
extern crate thiserror;

pub use types::{
    header::Header,
    istanbul::IstanbulExtra,
};
pub use istanbul::{
    get_epoch_number,
    get_epoch_last_block_number,
};
pub use state::State;
pub use errors::{Error, Kind};
pub use traits::{
    Storage,
    FromBytes,
    DefaultFrom
};
