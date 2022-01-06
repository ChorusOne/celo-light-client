pub mod client;
pub mod consensus;
pub mod errors;
pub mod header;
pub mod istanbul;
pub mod serialization;
pub mod state;
pub mod proof;
pub(crate) mod test_proofs;
//pub mod verify2;
pub mod verify;

#[cfg(test)]
mod integration;

pub use errors::Error;
pub use header::Header;
use istanbul::*;

#[cfg(any(feature = "bls-support"))]
pub mod bls;
#[cfg(any(feature = "bls-support"))]
use bls::*;
#[cfg(not(feature = "bls-support"))]
pub(crate) fn verify_aggregated_seal(
    _header_hash: H256,
    _validators: &[ValidatorData],
    _seal: &IstanbulAggregatedSeal,
) -> Result<(), Error> {
    Ok(())
}

use ethereum_types::*;
use sha3::{Digest, Keccak256};

pub fn istanbul_filtered_header(header: &Header, keep_seal: bool) -> Result<Header, Error> {
    let mut new_header = header.clone();
    let mut extra: IstanbulExtra = extract_istanbul_extra(&new_header)?;
    if !keep_seal {
        extra.seal = Vec::new();
    }
    extra.aggregated_seal = IstanbulAggregatedSeal::default();
    let payload = rlp::encode(&extra);
    let mut new_extra = new_header.extra[..IstanbulExtraVanity::len_bytes()].to_vec();
    new_extra.append(&mut payload.to_vec());
    new_header.extra = new_extra;
    Ok(new_header)
}

pub fn rlp_hash(header: &Header) -> H256 {
    let digest = Keccak256::digest(&rlp::encode(header));
    H256::from(digest.as_ref())
}

pub fn hash_header(header: &Header) -> H256 {
    if header.extra.len() >= IstanbulExtraVanity::len_bytes() {
        let istanbul_header = istanbul_filtered_header(header, true);
        if let Ok(ist_header) = istanbul_header {
            return rlp_hash(&ist_header);
        }
    }
    rlp_hash(header)
}

pub fn extract_istanbul_extra(h: &Header) -> Result<IstanbulExtra, Error> {
    IstanbulExtra::from_rlp(h.extra.as_slice())
}

// Retrieves the block number within an epoch. The return value will be 1-based.
// There is a special case if the number == 0. It is basically the last block of the 0th epoch,
// and should have a value of epoch_size
pub fn get_number_within_epoch(number: u64, epoch_size: u64) -> u64 {
    let number = number % epoch_size;
    if number == 0 {
        epoch_size
    } else {
        number
    }
}

pub fn get_epoch_number(number: u64, epoch_size: u64) -> u64 {
    let epoch_number = number / epoch_size;
    if is_last_block_of_epoch(number, epoch_size) {
        epoch_number
    } else {
        epoch_number + 1
    }
}

pub fn get_epoch_first_block_number(epoch_number: u64, epoch_size: u64) -> Option<u64> {
    if epoch_number == 0 {
        // no first block for epoch 0
        return None;
    }
    Some(((epoch_number - 1) * epoch_size) + 1)
}

pub fn get_epoch_last_block_number(epoch_number: u64, epoch_size: u64) -> u64 {
    if epoch_number == 0 {
        return 0;
    }
    // Epoch 0 is just the genesis bock, so epoch 1 starts at block 1 and ends at block epochSize
    // And from then on, it's epochSize more for each epoch
    epoch_number * epoch_size
}

pub fn is_last_block_of_epoch(number: u64, epoch_size: u64) -> bool {
    get_number_within_epoch(number, epoch_size) == epoch_size
}

pub fn min_quorum_size(total_validators: usize) -> usize {
    // non-float equivalent of:
    //  ((2.0*(total_validators as f64) / 3.0) as f64).ceil() as usize
    ((2 * total_validators) - 1 + 3) / 3
}

pub fn verify(_lcs: &consensus::LightConsensusState, _lcl: &client::LightClientState) -> Result<(), Error> {
    //todo!()
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn validates_quorum_size_math() {
        for (validator_set_size, expected_min_quorum_size) in vec![
            (1 as usize, 1 as usize),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 4),
            (6, 4),
            (7, 5),
        ]
        .iter()
        {
            assert_eq!(
                min_quorum_size(*validator_set_size),
                *expected_min_quorum_size
            );
        }
    }

    #[test]
    fn validates_epoch_math() {
        assert_eq!(
            vec![
                get_epoch_number(0, 3),
                get_epoch_number(3, 3),
                get_epoch_number(4, 3)
            ],
            vec![0, 1, 2]
        );

        assert_eq!(
            vec![
                get_epoch_first_block_number(0, 3),
                get_epoch_first_block_number(9, 3)
            ],
            vec![None, Some(25)]
        );

        assert_eq!(
            vec![
                get_epoch_last_block_number(0, 3),
                get_epoch_last_block_number(9, 3)
            ],
            vec![0, 27]
        );
    }

    #[test]
    fn header_hash_1() {
        let hash =
            H256::from_str("0x038c24a0954803292abc95ef76082df8917510b0d31eda67ab4eb9dad91f8777")
                .unwrap();
        let header = Header{parent_hash: H256::from_str("00000000000000000000000000000000000000000000000024cfd42fb0afb447").unwrap(), coinbase: H160::from_str("0000000000000000000000007c713f392c17bd65").unwrap(), root: H256::from_str("00000000000000000000000000000000000000000000000011849c91e6160bf9").unwrap(), tx_hash: H256::from_str("00000000000000000000000000000000000000000000000010458f5f8c969c53").unwrap(), receipt_hash: H256::from_str("00000000000000000000000000000000000000000000000044c4e1cdbe2158d4").unwrap(), bloom: Bloom::from_str("5a351c5424ecc5b05e04c2f42ee8f9ecd6f11f874b0315babda9c6cd0741794c9165b6240a744a21fb9329698f82d68611c98d962c400a23a0037398d2ab0334dc89b631a00810c729a7edc91bdf22e51ee237d90341535703a32056dbfb4c90f23f17220acccd6fe647fa6b4df778ad1410ac4c21bfb24bb62d862a4ea805c9de7f4716ce13e3f4c2efeba47f8943c0bcc5ceda0137b0033a4a2ca4c2e7693f5491a9b7f692c95499a50848a44419efdc163c4ca195170b68f5327fba09d639fe09d969f49f3582d82d5a1ad3bd9c9354d794ee137d0e4538735e2fc3e7477ff64ef42be6b2c48e0262a362cdbda8f936c445a5713fa41423b0608b16b14bbd").unwrap(), number: U64::from(6905198215718552558 as u64), gas_used: U256::from(9305342113105117570 as u64), time: U256::from(11715508990386030326 as u64), extra: hex::decode("4d6e821ac8016d9062669ba9249dd0534d1e9e152320964aa5919faf62795e37b2324a4a4dd378a95cadc0a7501c885aea1bdf21922ff2a55714b464aee5da54").unwrap()};

        assert_eq!(hash, hash_header(&header));
    }
    #[test]
    fn header_hash_2() {
        let hash =
            H256::from_str("0xa7bd93e8c3c31d661018265ee832e8737390d0f2a86d2207bec4c4d4e8571fea")
                .unwrap();
        let header = Header{parent_hash: H256::from_str("00000000000000000000000000000000000000000000000004f51c8d3cad4cd4").unwrap(), coinbase: H160::from_str("000000000000000000000000709d3a3d7969f2a2").unwrap(), root: H256::from_str("00000000000000000000000000000000000000000000000060ac98bfde2d97e0").unwrap(), tx_hash: H256::from_str("0000000000000000000000000000000000000000000000004178b842922c98d9").unwrap(), receipt_hash: H256::from_str("0000000000000000000000000000000000000000000000003d892f53d8083efe").unwrap(), bloom: Bloom::from_str("91b5064b99bc8e7f461cea2c86a7a7b66a891b93bdd480279d7bde19fbc459fd20a7a7c17b126d4643ec33161ee0c0c9a440a2e17e0c1c5b24965b361672b09fb30d54a0b41ccc7de93085709cb172b785556452abbc3ede2f13d79b211786d916a85e94734c4429b2c91a2e8172d509031350e9da3bdf56971b106e03c1301cc60f44604a19c118bb188a8e375e1fd6935d0b923f6750711cf6029631783770ab57a094965fd2b36de63bf152ada5f2e87d4e128bb4091b2c9eb1b31f63f561bbdc3f1d607adc39e2643a287b01beb10aae082cd9a25b77399a4e60b32cd3b0e281120e4444595953a23f7b948e7334147dbee61757e03ac730066cc005bc74").unwrap(), number: U64::from(8759088914100798478 as u64), gas_used: U256::from(4716044633133310045 as u64), time: U256::from(15602243294024492957 as u64), extra: hex::decode("0ab421bd50f38f00b8f590957c175b61a7c296e603f476b3b10c281fcacb8acdc453843486bbbf128ac5eba78194ba65770c3c5ae81de6198ce9f78a65b6c9b3").unwrap()};

        assert_eq!(hash, hash_header(&header));
    }

    #[test]
    fn header_hash_3() {
        let hash =
            H256::from_str("0x916f9843415d9534935df2f1a623c7fcd057ac2269a4bc4900cbb55589bd1e58")
                .unwrap();
        let header = Header{parent_hash: H256::from_str("00000000000000000000000000000000000000000000000067c1caf98fce5fed").unwrap(), coinbase: H160::from_str("00000000000000000000000019c0487fd937a206").unwrap(), root: H256::from_str("0000000000000000000000000000000000000000000000000bdbd3b900527e06").unwrap(), tx_hash: H256::from_str("00000000000000000000000000000000000000000000000023b81143fff0f675").unwrap(), receipt_hash: H256::from_str("00000000000000000000000000000000000000000000000023aa2f717bea4bca").unwrap(), bloom: Bloom::from_str("21b4d0914ed15f3a50666926e3f411e7b379b9bb0de5bef605831e7131d4b5bf274481d47751be53fac7ed2bf3d84a2f778bb09776365fd95f29af413e1de49abb5e0a35d49f78ae9b38d46a0b0d27fa3d95d3e9022f8d28d7caff35a8fb180cc6fdf24d9ded8642b446807d5d80760fff524ea2143b4452bb7ff273ac75cc5405a5db65ff299db7816b26a063101dae8d46ccb8aaf2e9ffad86d46755f0070af3e12a9e8d787af7da95e587bb1f79816a41801c4254796b53361571d427990a6925acab66639f2a3c08d48cad24320b6fccf46a115745c09e4d13ba460d61eb4dff47d9b45f9f6f6b1919d095ddbf45ff3311e29d3a6b7d1bae0987e06388ad").unwrap(), number: U64::from(7117619510059526286 as u64), gas_used: U256::from(6151569940903227990 as u64), time: U256::from(5812628570597096534 as u64), extra: hex::decode("fb3973baeea8acf934e26474b5ac2ed42233af643ccb917f1959ab4df65c74b7d6e8be81f347aea284f346f1a42ff63eba70525438a20e383106f6d3740be27d").unwrap()};

        assert_eq!(hash, hash_header(&header));
    }
}
