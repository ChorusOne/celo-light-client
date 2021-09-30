use crate::{Error, Kind};
use ethereum_types::*;
use std::convert::TryFrom;

/// https://pkg.go.dev/github.com/celo-org/celo-blockchain/core/types#Header
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(
    any(test, feature = "serialize"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(test, feature = "serialize"), serde(rename_all = "camelCase"))]
pub struct Header {
    pub parent_hash: H256,
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "miner"))]
    pub coinbase: Address,
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "stateRoot"))]
    pub root: H256,
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "transactionsRoot"))]
    pub tx_hash: H256,
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "receiptsRoot"))]
    pub receipt_hash: H256,
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "logsBloom"))]
    pub bloom: Bloom,
    pub number: U64,
    pub gas_used: U256,
    #[cfg_attr(any(test, feature = "serialize"), serde(rename= "timestamp"))]
    pub time: U256,
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "extraData", with = "crate::serialization::hexstring"))]
    pub extra: Vec<u8>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            parent_hash: H256::default(),
            coinbase: Address::default(),
            root: H256::default(),
            tx_hash: H256::default(),
            receipt_hash: H256::default(),
            bloom: Bloom::default(),
            number: U64::default(),
            gas_used: U256::default(),
            time: U256::default(),
            extra: Vec::default(),
        }
    }
}
impl rlp::Decodable for Header {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        Ok(Header {
            parent_hash: rlp.val_at(0)?,
            coinbase: rlp.val_at(1)?,
            root: rlp.val_at(2)?,
            tx_hash: rlp.val_at(3)?,
            receipt_hash: rlp.val_at(4)?,
            bloom: rlp.val_at(5)?,
            number: rlp.val_at(6)?,
            gas_used: rlp.val_at(7)?,
            time: rlp.val_at(8)?,
            extra: rlp.val_at(9)?,
        })
    }
}
impl rlp::Encodable for Header {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_list(10);
        s.append(&self.parent_hash.as_ref());
        s.append(&self.coinbase.as_ref());
        s.append(&self.root.as_ref());
        s.append(&self.tx_hash.as_ref());
        s.append(&self.receipt_hash.as_ref());
        s.append(&self.bloom.as_ref());
        s.append(&self.number);
        s.append(&self.gas_used);
        s.append(&self.time);
        s.append(&self.extra);
    }
}

#[cfg(feature = "web3_support")]
impl TryFrom<web3::types::Block<H256>> for Header {
    type Error = Error;
    fn try_from(blk: web3::types::Block<H256>) -> Result<Self, Self::Error> {
        let s = Self {
            bloom: blk.logs_bloom.ok_or_else(|| Kind::MissingField {
                field: String::from("logs_bloom"),
            })?,
            coinbase: blk.author,
            extra: blk.extra_data.0,
            gas_used: blk.gas_used,
            number: blk.number.ok_or_else(|| Kind::MissingField {
                field: String::from("number"),
            })?,
            parent_hash: blk.parent_hash,
            receipt_hash: blk.receipts_root,
            tx_hash: blk.transactions_root,
            root: blk.state_root,
            time: blk.timestamp,
        };
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn header_rlp_1() {
        let hexa = String::from("f901f9a00000000000000000000000000000000000000000000000006ef2765f689d531794000000000000000000000000205d0ede64bc98a7a0000000000000000000000000000000000000000000000000596cc2d32a4b86b2a00000000000000000000000000000000000000000000000003a135a76422338e6a000000000000000000000000000000000000000000000000071f660f72de596d6b90100e21748303ff4c3465e29a0dca46ac3a0d570133bc21571f5f54a643105513fd8429b194ae1ba9f2a8386ae72b3661fe82c2ac64dbf46151824e2305312637c99c2b87d6df0dd5ef2836719072d672f61faa0be86f457921ec37c0e460cf0d703d07dc5f5661a450133409a712b6a66abd7ba65224ca26978d9a5ce62feb11a838f16622eb54cee3e6755229c183c1933a916585e183bb709192db0e6c8d709b9d7383b993c69499945b0b0cfe3dc2ec1fc9610d7eeb19d0916a27423c29e442f0e31c2c9539660f05fb37e45cd1bf0bcfc1a346c434f11cf00416fa61695b9202e68363a1c787f4784e0aed9c78564184e131f1e7c811fbd885bcc6734e8933f882f54c7f8f9525df288539f8c27d6e1d77a8893949d0f842f877cb8403cb2d9fdedae93db2495718f580b8c8f2becc6aa76c3b0f948afbe6127a5cfd56bc11b3c2ea837a1c669a822f76864ec2ae0cd6bfddf21b1144474bfd37cd206");
        let json = String::from(
            r#"{"parentHash":"0x0000000000000000000000000000000000000000000000006ef2765f689d5317","miner":"0x000000000000000000000000205d0ede64bc98a7","stateRoot":"0x000000000000000000000000000000000000000000000000596cc2d32a4b86b2","transactionsRoot":"0x0000000000000000000000000000000000000000000000003a135a76422338e6","receiptsRoot":"0x00000000000000000000000000000000000000000000000071f660f72de596d6","logsBloom":"0xe21748303ff4c3465e29a0dca46ac3a0d570133bc21571f5f54a643105513fd8429b194ae1ba9f2a8386ae72b3661fe82c2ac64dbf46151824e2305312637c99c2b87d6df0dd5ef2836719072d672f61faa0be86f457921ec37c0e460cf0d703d07dc5f5661a450133409a712b6a66abd7ba65224ca26978d9a5ce62feb11a838f16622eb54cee3e6755229c183c1933a916585e183bb709192db0e6c8d709b9d7383b993c69499945b0b0cfe3dc2ec1fc9610d7eeb19d0916a27423c29e442f0e31c2c9539660f05fb37e45cd1bf0bcfc1a346c434f11cf00416fa61695b9202e68363a1c787f4784e0aed9c78564184e131f1e7c811fbd885bcc6734e8933f","number":"0x2f54c7f8f9525df2","gasUsed":"0x539f8c27d6e1d77a","timestamp":"0x93949d0f842f877c","extraData":"0x3cb2d9fdedae93db2495718f580b8c8f2becc6aa76c3b0f948afbe6127a5cfd56bc11b3c2ea837a1c669a822f76864ec2ae0cd6bfddf21b1144474bfd37cd206","hash":"0xceb7da4220df7d209ca399a34ac22a25e7f73abf617cd52b1a63f16eb780cd41"}"#,
        );
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
    }
}
