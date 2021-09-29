pub mod errors;
pub mod header;
pub mod istanbul;
pub mod serialization;
pub mod state;
pub mod client;
pub mod consensus;

pub use errors::{Error, Kind};
pub use header::Header;
use istanbul::*;

use ethereum_types::*;
use sha3::{Digest, Keccak256};
use std::convert::TryInto;

pub fn istanbul_filtered_header(header: &Header, keep_seal: bool) -> Result<Header, Error> {
    let mut new_header = header.clone();
    let mut extra: IstanbulExtra =
        rlp::decode(&new_header.extra).map_err(|_e| Kind::RlpDecodeError)?;
    if !keep_seal {
        extra.seal = Vec::new();
    }
    extra.aggregated_seal = IstanbulAggregatedSeal::default();
    let t: [u8; IstanbulExtraVanity::len_bytes()] = new_header
        .extra
        .as_slice()
        .try_into()
        .map_err(|_e| Kind::InvalidDataLength {
            current: new_header.extra.len(),
            expected: IstanbulExtraVanity::len_bytes(),
        })?;
    let vanity = IstanbulExtraVanity::from(t);
    let payload = extra.to_rlp(&vanity);
    new_header.extra = payload;
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
        let hexa = String::from("f901f9a00000000000000000000000000000000000000000000000006ef2765f689d531794000000000000000000000000205d0ede64bc98a7a0000000000000000000000000000000000000000000000000596cc2d32a4b86b2a00000000000000000000000000000000000000000000000003a135a76422338e6a000000000000000000000000000000000000000000000000071f660f72de596d6b90100e21748303ff4c3465e29a0dca46ac3a0d570133bc21571f5f54a643105513fd8429b194ae1ba9f2a8386ae72b3661fe82c2ac64dbf46151824e2305312637c99c2b87d6df0dd5ef2836719072d672f61faa0be86f457921ec37c0e460cf0d703d07dc5f5661a450133409a712b6a66abd7ba65224ca26978d9a5ce62feb11a838f16622eb54cee3e6755229c183c1933a916585e183bb709192db0e6c8d709b9d7383b993c69499945b0b0cfe3dc2ec1fc9610d7eeb19d0916a27423c29e442f0e31c2c9539660f05fb37e45cd1bf0bcfc1a346c434f11cf00416fa61695b9202e68363a1c787f4784e0aed9c78564184e131f1e7c811fbd885bcc6734e8933f882f54c7f8f9525df288539f8c27d6e1d77a8893949d0f842f877cb8403cb2d9fdedae93db2495718f580b8c8f2becc6aa76c3b0f948afbe6127a5cfd56bc11b3c2ea837a1c669a822f76864ec2ae0cd6bfddf21b1144474bfd37cd206");
        let json = String::from(
            r#"{"parentHash":"0x0000000000000000000000000000000000000000000000006ef2765f689d5317","miner":"0x000000000000000000000000205d0ede64bc98a7","stateRoot":"0x000000000000000000000000000000000000000000000000596cc2d32a4b86b2","transactionsRoot":"0x0000000000000000000000000000000000000000000000003a135a76422338e6","receiptsRoot":"0x00000000000000000000000000000000000000000000000071f660f72de596d6","logsBloom":"0xe21748303ff4c3465e29a0dca46ac3a0d570133bc21571f5f54a643105513fd8429b194ae1ba9f2a8386ae72b3661fe82c2ac64dbf46151824e2305312637c99c2b87d6df0dd5ef2836719072d672f61faa0be86f457921ec37c0e460cf0d703d07dc5f5661a450133409a712b6a66abd7ba65224ca26978d9a5ce62feb11a838f16622eb54cee3e6755229c183c1933a916585e183bb709192db0e6c8d709b9d7383b993c69499945b0b0cfe3dc2ec1fc9610d7eeb19d0916a27423c29e442f0e31c2c9539660f05fb37e45cd1bf0bcfc1a346c434f11cf00416fa61695b9202e68363a1c787f4784e0aed9c78564184e131f1e7c811fbd885bcc6734e8933f","number":"0x2f54c7f8f9525df2","gasUsed":"0x539f8c27d6e1d77a","timestamp":"0x93949d0f842f877c","extraData":"0x3cb2d9fdedae93db2495718f580b8c8f2becc6aa76c3b0f948afbe6127a5cfd56bc11b3c2ea837a1c669a822f76864ec2ae0cd6bfddf21b1144474bfd37cd206","hash":"0xceb7da4220df7d209ca399a34ac22a25e7f73abf617cd52b1a63f16eb780cd41"}"#,
        );
        let hash =
            H256::from_str("ceb7da4220df7d209ca399a34ac22a25e7f73abf617cd52b1a63f16eb780cd41")
                .unwrap();
        let expected = Header{parent_hash: H256::from_str("0000000000000000000000000000000000000000000000006ef2765f689d5317").unwrap(), coinbase: H160::from_str("000000000000000000000000205d0ede64bc98a7").unwrap(), root: H256::from_str("000000000000000000000000000000000000000000000000596cc2d32a4b86b2").unwrap(), tx_hash: H256::from_str("0000000000000000000000000000000000000000000000003a135a76422338e6").unwrap(), receipt_hash: H256::from_str("00000000000000000000000000000000000000000000000071f660f72de596d6").unwrap(), bloom: Bloom::from_str("e21748303ff4c3465e29a0dca46ac3a0d570133bc21571f5f54a643105513fd8429b194ae1ba9f2a8386ae72b3661fe82c2ac64dbf46151824e2305312637c99c2b87d6df0dd5ef2836719072d672f61faa0be86f457921ec37c0e460cf0d703d07dc5f5661a450133409a712b6a66abd7ba65224ca26978d9a5ce62feb11a838f16622eb54cee3e6755229c183c1933a916585e183bb709192db0e6c8d709b9d7383b993c69499945b0b0cfe3dc2ec1fc9610d7eeb19d0916a27423c29e442f0e31c2c9539660f05fb37e45cd1bf0bcfc1a346c434f11cf00416fa61695b9202e68363a1c787f4784e0aed9c78564184e131f1e7c811fbd885bcc6734e8933f").unwrap(), number: U64::from(3410570689975049714 as u64), gas_used: U256::from(6025688929181751162 as u64), time: U256::from(10634297310096361340 as u64), extra: hex::decode("3cb2d9fdedae93db2495718f580b8c8f2becc6aa76c3b0f948afbe6127a5cfd56bc11b3c2ea837a1c669a822f76864ec2ae0cd6bfddf21b1144474bfd37cd206").unwrap()};

        let rlp_header: Header = rlp::decode(hex::decode(&hexa).unwrap().as_slice()).unwrap();
        let json_header: Header = serde_json::from_str(&json).unwrap();

        assert_eq!(expected.parent_hash, rlp_header.parent_hash, "parent_hash");
        assert_eq!(expected.coinbase, rlp_header.coinbase, "coinbase");
        assert_eq!(expected.root, rlp_header.root, "root");
        assert_eq!(expected.tx_hash, rlp_header.tx_hash, "tx_hash");
        assert_eq!(
            expected.receipt_hash, rlp_header.receipt_hash,
            "receipt_hash"
        );
        assert_eq!(expected.bloom, rlp_header.bloom, "bloom");
        assert_eq!(expected.number, rlp_header.number, "number");
        assert_eq!(expected.gas_used, rlp_header.gas_used, "gas_used");
        assert_eq!(expected.time, rlp_header.time, "time");
        assert_eq!(expected.extra, rlp_header.extra, "extra");

        assert_eq!(expected.parent_hash, json_header.parent_hash, "parent_hash");
        assert_eq!(expected.coinbase, json_header.coinbase, "coinbase");
        assert_eq!(expected.root, json_header.root, "root");
        assert_eq!(expected.tx_hash, json_header.tx_hash, "tx_hash");
        assert_eq!(
            expected.receipt_hash, json_header.receipt_hash,
            "receipt_hash"
        );
        assert_eq!(expected.bloom, json_header.bloom, "bloom");
        assert_eq!(expected.number, json_header.number, "number");
        assert_eq!(expected.gas_used, json_header.gas_used, "gas_used");
        assert_eq!(expected.time, json_header.time, "time");
        assert_eq!(expected.extra, rlp_header.extra, "extra");

        assert_eq!(hash, hash_header(&rlp_header));
    }
    #[test]
    fn header_hash_2() {
        let hexa = String::from("f901f9a000000000000000000000000000000000000000000000000024cfd42fb0afb447940000000000000000000000007c713f392c17bd65a000000000000000000000000000000000000000000000000011849c91e6160bf9a000000000000000000000000000000000000000000000000010458f5f8c969c53a000000000000000000000000000000000000000000000000044c4e1cdbe2158d4b901005a351c5424ecc5b05e04c2f42ee8f9ecd6f11f874b0315babda9c6cd0741794c9165b6240a744a21fb9329698f82d68611c98d962c400a23a0037398d2ab0334dc89b631a00810c729a7edc91bdf22e51ee237d90341535703a32056dbfb4c90f23f17220acccd6fe647fa6b4df778ad1410ac4c21bfb24bb62d862a4ea805c9de7f4716ce13e3f4c2efeba47f8943c0bcc5ceda0137b0033a4a2ca4c2e7693f5491a9b7f692c95499a50848a44419efdc163c4ca195170b68f5327fba09d639fe09d969f49f3582d82d5a1ad3bd9c9354d794ee137d0e4538735e2fc3e7477ff64ef42be6b2c48e0262a362cdbda8f936c445a5713fa41423b0608b16b14bbd885fd431311f90c3ee88812337599d1d018288a295d95af6dac6f6b8404d6e821ac8016d9062669ba9249dd0534d1e9e152320964aa5919faf62795e37b2324a4a4dd378a95cadc0a7501c885aea1bdf21922ff2a55714b464aee5da54");
        let json = String::from(
            r#"{"parentHash":"0x00000000000000000000000000000000000000000000000024cfd42fb0afb447","miner":"0x0000000000000000000000007c713f392c17bd65","stateRoot":"0x00000000000000000000000000000000000000000000000011849c91e6160bf9","transactionsRoot":"0x00000000000000000000000000000000000000000000000010458f5f8c969c53","receiptsRoot":"0x00000000000000000000000000000000000000000000000044c4e1cdbe2158d4","logsBloom":"0x5a351c5424ecc5b05e04c2f42ee8f9ecd6f11f874b0315babda9c6cd0741794c9165b6240a744a21fb9329698f82d68611c98d962c400a23a0037398d2ab0334dc89b631a00810c729a7edc91bdf22e51ee237d90341535703a32056dbfb4c90f23f17220acccd6fe647fa6b4df778ad1410ac4c21bfb24bb62d862a4ea805c9de7f4716ce13e3f4c2efeba47f8943c0bcc5ceda0137b0033a4a2ca4c2e7693f5491a9b7f692c95499a50848a44419efdc163c4ca195170b68f5327fba09d639fe09d969f49f3582d82d5a1ad3bd9c9354d794ee137d0e4538735e2fc3e7477ff64ef42be6b2c48e0262a362cdbda8f936c445a5713fa41423b0608b16b14bbd","number":"0x5fd431311f90c3ee","gasUsed":"0x812337599d1d0182","timestamp":"0xa295d95af6dac6f6","extraData":"0x4d6e821ac8016d9062669ba9249dd0534d1e9e152320964aa5919faf62795e37b2324a4a4dd378a95cadc0a7501c885aea1bdf21922ff2a55714b464aee5da54","hash":"0x038c24a0954803292abc95ef76082df8917510b0d31eda67ab4eb9dad91f8777"}"#,
        );
        let hash =
            H256::from_str("038c24a0954803292abc95ef76082df8917510b0d31eda67ab4eb9dad91f8777")
                .unwrap();
        let expected = Header{parent_hash: H256::from_str("00000000000000000000000000000000000000000000000024cfd42fb0afb447").unwrap(), coinbase: H160::from_str("0000000000000000000000007c713f392c17bd65").unwrap(), root: H256::from_str("00000000000000000000000000000000000000000000000011849c91e6160bf9").unwrap(), tx_hash: H256::from_str("00000000000000000000000000000000000000000000000010458f5f8c969c53").unwrap(), receipt_hash: H256::from_str("00000000000000000000000000000000000000000000000044c4e1cdbe2158d4").unwrap(), bloom: Bloom::from_str("5a351c5424ecc5b05e04c2f42ee8f9ecd6f11f874b0315babda9c6cd0741794c9165b6240a744a21fb9329698f82d68611c98d962c400a23a0037398d2ab0334dc89b631a00810c729a7edc91bdf22e51ee237d90341535703a32056dbfb4c90f23f17220acccd6fe647fa6b4df778ad1410ac4c21bfb24bb62d862a4ea805c9de7f4716ce13e3f4c2efeba47f8943c0bcc5ceda0137b0033a4a2ca4c2e7693f5491a9b7f692c95499a50848a44419efdc163c4ca195170b68f5327fba09d639fe09d969f49f3582d82d5a1ad3bd9c9354d794ee137d0e4538735e2fc3e7477ff64ef42be6b2c48e0262a362cdbda8f936c445a5713fa41423b0608b16b14bbd").unwrap(), number: U64::from(6905198215718552558 as u64), gas_used: U256::from(9305342113105117570 as u64), time: U256::from(11715508990386030326 as u64), extra: hex::decode("4d6e821ac8016d9062669ba9249dd0534d1e9e152320964aa5919faf62795e37b2324a4a4dd378a95cadc0a7501c885aea1bdf21922ff2a55714b464aee5da54").unwrap()};

        let rlp_header: Header = rlp::decode(hex::decode(&hexa).unwrap().as_slice()).unwrap();
        let json_header: Header = serde_json::from_str(&json).unwrap();

        assert_eq!(expected.parent_hash, rlp_header.parent_hash, "parent_hash");
        assert_eq!(expected.coinbase, rlp_header.coinbase, "coinbase");
        assert_eq!(expected.root, rlp_header.root, "root");
        assert_eq!(expected.tx_hash, rlp_header.tx_hash, "tx_hash");
        assert_eq!(
            expected.receipt_hash, rlp_header.receipt_hash,
            "receipt_hash"
        );
        assert_eq!(expected.bloom, rlp_header.bloom, "bloom");
        assert_eq!(expected.number, rlp_header.number, "number");
        assert_eq!(expected.gas_used, rlp_header.gas_used, "gas_used");
        assert_eq!(expected.time, rlp_header.time, "time");
        assert_eq!(expected.extra, rlp_header.extra, "extra");

        assert_eq!(expected.parent_hash, json_header.parent_hash, "parent_hash");
        assert_eq!(expected.coinbase, json_header.coinbase, "coinbase");
        assert_eq!(expected.root, json_header.root, "root");
        assert_eq!(expected.tx_hash, json_header.tx_hash, "tx_hash");
        assert_eq!(
            expected.receipt_hash, json_header.receipt_hash,
            "receipt_hash"
        );
        assert_eq!(expected.bloom, json_header.bloom, "bloom");
        assert_eq!(expected.number, json_header.number, "number");
        assert_eq!(expected.gas_used, json_header.gas_used, "gas_used");
        assert_eq!(expected.time, json_header.time, "time");
        assert_eq!(expected.extra, rlp_header.extra, "extra");

        assert_eq!(hash, hash_header(&rlp_header));
    }
}
