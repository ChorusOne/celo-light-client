use ethereum_types::*;

/// https://pkg.go.dev/github.com/celo-org/celo-blockchain/core/types#Header
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(any(test, feature = "serialize"), derive(serde::Deserialize, serde::Serialize))]
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
    #[cfg_attr(any(test, feature = "serialize"), serde(rename = "timestamp"))]
    pub time: U256,
    #[cfg_attr(
        any(test, feature = "serialize"),
        serde(rename = "extraData", with = "crate::serialization::hexstring")
    )]
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

#[cfg(feature = "web3-support")]
impl std::convert::TryFrom<web3::types::Block<H256>> for Header {
    type Error = super::errors::web3::MissingFieldErr;
    fn try_from(blk: web3::types::Block<H256>) -> Result<Self, Self::Error> {
        let s = Self {
            bloom: blk.logs_bloom.ok_or_else(|| super::errors::web3::MissingFieldErr(String::from("logs_bloom")))?,
            coinbase: blk.author,
            extra: blk.extra_data.0,
            gas_used: blk.gas_used,
            number: blk.number.ok_or_else(|| super::errors::web3::MissingFieldErr(String::from("number")))?,
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
    fn header_1() {
        let hexa = String::from("f901f9a000000000000000000000000000000000000000000000000024cfd42fb0afb447940000000000000000000000007c713f392c17bd65a000000000000000000000000000000000000000000000000011849c91e6160bf9a000000000000000000000000000000000000000000000000010458f5f8c969c53a000000000000000000000000000000000000000000000000044c4e1cdbe2158d4b901005a351c5424ecc5b05e04c2f42ee8f9ecd6f11f874b0315babda9c6cd0741794c9165b6240a744a21fb9329698f82d68611c98d962c400a23a0037398d2ab0334dc89b631a00810c729a7edc91bdf22e51ee237d90341535703a32056dbfb4c90f23f17220acccd6fe647fa6b4df778ad1410ac4c21bfb24bb62d862a4ea805c9de7f4716ce13e3f4c2efeba47f8943c0bcc5ceda0137b0033a4a2ca4c2e7693f5491a9b7f692c95499a50848a44419efdc163c4ca195170b68f5327fba09d639fe09d969f49f3582d82d5a1ad3bd9c9354d794ee137d0e4538735e2fc3e7477ff64ef42be6b2c48e0262a362cdbda8f936c445a5713fa41423b0608b16b14bbd885fd431311f90c3ee88812337599d1d018288a295d95af6dac6f6b8404d6e821ac8016d9062669ba9249dd0534d1e9e152320964aa5919faf62795e37b2324a4a4dd378a95cadc0a7501c885aea1bdf21922ff2a55714b464aee5da54");
        let json = String::from(
            r#"{"parentHash":"0x00000000000000000000000000000000000000000000000024cfd42fb0afb447","miner":"0x0000000000000000000000007c713f392c17bd65","stateRoot":"0x00000000000000000000000000000000000000000000000011849c91e6160bf9","transactionsRoot":"0x00000000000000000000000000000000000000000000000010458f5f8c969c53","receiptsRoot":"0x00000000000000000000000000000000000000000000000044c4e1cdbe2158d4","logsBloom":"0x5a351c5424ecc5b05e04c2f42ee8f9ecd6f11f874b0315babda9c6cd0741794c9165b6240a744a21fb9329698f82d68611c98d962c400a23a0037398d2ab0334dc89b631a00810c729a7edc91bdf22e51ee237d90341535703a32056dbfb4c90f23f17220acccd6fe647fa6b4df778ad1410ac4c21bfb24bb62d862a4ea805c9de7f4716ce13e3f4c2efeba47f8943c0bcc5ceda0137b0033a4a2ca4c2e7693f5491a9b7f692c95499a50848a44419efdc163c4ca195170b68f5327fba09d639fe09d969f49f3582d82d5a1ad3bd9c9354d794ee137d0e4538735e2fc3e7477ff64ef42be6b2c48e0262a362cdbda8f936c445a5713fa41423b0608b16b14bbd","number":"0x5fd431311f90c3ee","gasUsed":"0x812337599d1d0182","timestamp":"0xa295d95af6dac6f6","extraData":"0x4d6e821ac8016d9062669ba9249dd0534d1e9e152320964aa5919faf62795e37b2324a4a4dd378a95cadc0a7501c885aea1bdf21922ff2a55714b464aee5da54","hash":"0x038c24a0954803292abc95ef76082df8917510b0d31eda67ab4eb9dad91f8777"}"#,
        );
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
    }

    #[test]
    fn header_2() {
        let hexa = String::from("f901f9a000000000000000000000000000000000000000000000000004f51c8d3cad4cd494000000000000000000000000709d3a3d7969f2a2a000000000000000000000000000000000000000000000000060ac98bfde2d97e0a00000000000000000000000000000000000000000000000004178b842922c98d9a00000000000000000000000000000000000000000000000003d892f53d8083efeb9010091b5064b99bc8e7f461cea2c86a7a7b66a891b93bdd480279d7bde19fbc459fd20a7a7c17b126d4643ec33161ee0c0c9a440a2e17e0c1c5b24965b361672b09fb30d54a0b41ccc7de93085709cb172b785556452abbc3ede2f13d79b211786d916a85e94734c4429b2c91a2e8172d509031350e9da3bdf56971b106e03c1301cc60f44604a19c118bb188a8e375e1fd6935d0b923f6750711cf6029631783770ab57a094965fd2b36de63bf152ada5f2e87d4e128bb4091b2c9eb1b31f63f561bbdc3f1d607adc39e2643a287b01beb10aae082cd9a25b77399a4e60b32cd3b0e281120e4444595953a23f7b948e7334147dbee61757e03ac730066cc005bc7488798e88f5b10a440e884172c19b6ecc605d88d8864d9c978ce79db8400ab421bd50f38f00b8f590957c175b61a7c296e603f476b3b10c281fcacb8acdc453843486bbbf128ac5eba78194ba65770c3c5ae81de6198ce9f78a65b6c9b3");
        let json = String::from(
            r#"{"parentHash":"0x00000000000000000000000000000000000000000000000004f51c8d3cad4cd4","miner":"0x000000000000000000000000709d3a3d7969f2a2","stateRoot":"0x00000000000000000000000000000000000000000000000060ac98bfde2d97e0","transactionsRoot":"0x0000000000000000000000000000000000000000000000004178b842922c98d9","receiptsRoot":"0x0000000000000000000000000000000000000000000000003d892f53d8083efe","logsBloom":"0x91b5064b99bc8e7f461cea2c86a7a7b66a891b93bdd480279d7bde19fbc459fd20a7a7c17b126d4643ec33161ee0c0c9a440a2e17e0c1c5b24965b361672b09fb30d54a0b41ccc7de93085709cb172b785556452abbc3ede2f13d79b211786d916a85e94734c4429b2c91a2e8172d509031350e9da3bdf56971b106e03c1301cc60f44604a19c118bb188a8e375e1fd6935d0b923f6750711cf6029631783770ab57a094965fd2b36de63bf152ada5f2e87d4e128bb4091b2c9eb1b31f63f561bbdc3f1d607adc39e2643a287b01beb10aae082cd9a25b77399a4e60b32cd3b0e281120e4444595953a23f7b948e7334147dbee61757e03ac730066cc005bc74","number":"0x798e88f5b10a440e","gasUsed":"0x4172c19b6ecc605d","timestamp":"0xd8864d9c978ce79d","extraData":"0x0ab421bd50f38f00b8f590957c175b61a7c296e603f476b3b10c281fcacb8acdc453843486bbbf128ac5eba78194ba65770c3c5ae81de6198ce9f78a65b6c9b3","hash":"0xa7bd93e8c3c31d661018265ee832e8737390d0f2a86d2207bec4c4d4e8571fea"}"#,
        );
        let expected = Header{parent_hash: H256::from_str("00000000000000000000000000000000000000000000000004f51c8d3cad4cd4").unwrap(), coinbase: H160::from_str("000000000000000000000000709d3a3d7969f2a2").unwrap(), root: H256::from_str("00000000000000000000000000000000000000000000000060ac98bfde2d97e0").unwrap(), tx_hash: H256::from_str("0000000000000000000000000000000000000000000000004178b842922c98d9").unwrap(), receipt_hash: H256::from_str("0000000000000000000000000000000000000000000000003d892f53d8083efe").unwrap(), bloom: Bloom::from_str("91b5064b99bc8e7f461cea2c86a7a7b66a891b93bdd480279d7bde19fbc459fd20a7a7c17b126d4643ec33161ee0c0c9a440a2e17e0c1c5b24965b361672b09fb30d54a0b41ccc7de93085709cb172b785556452abbc3ede2f13d79b211786d916a85e94734c4429b2c91a2e8172d509031350e9da3bdf56971b106e03c1301cc60f44604a19c118bb188a8e375e1fd6935d0b923f6750711cf6029631783770ab57a094965fd2b36de63bf152ada5f2e87d4e128bb4091b2c9eb1b31f63f561bbdc3f1d607adc39e2643a287b01beb10aae082cd9a25b77399a4e60b32cd3b0e281120e4444595953a23f7b948e7334147dbee61757e03ac730066cc005bc74").unwrap(), number: U64::from(8759088914100798478 as u64), gas_used: U256::from(4716044633133310045 as u64), time: U256::from(15602243294024492957 as u64), extra: hex::decode("0ab421bd50f38f00b8f590957c175b61a7c296e603f476b3b10c281fcacb8acdc453843486bbbf128ac5eba78194ba65770c3c5ae81de6198ce9f78a65b6c9b3").unwrap()};

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

    #[test]
    fn header_3() {
        let hexa = String::from("f901f9a000000000000000000000000000000000000000000000000067c1caf98fce5fed9400000000000000000000000019c0487fd937a206a00000000000000000000000000000000000000000000000000bdbd3b900527e06a000000000000000000000000000000000000000000000000023b81143fff0f675a000000000000000000000000000000000000000000000000023aa2f717bea4bcab9010021b4d0914ed15f3a50666926e3f411e7b379b9bb0de5bef605831e7131d4b5bf274481d47751be53fac7ed2bf3d84a2f778bb09776365fd95f29af413e1de49abb5e0a35d49f78ae9b38d46a0b0d27fa3d95d3e9022f8d28d7caff35a8fb180cc6fdf24d9ded8642b446807d5d80760fff524ea2143b4452bb7ff273ac75cc5405a5db65ff299db7816b26a063101dae8d46ccb8aaf2e9ffad86d46755f0070af3e12a9e8d787af7da95e587bb1f79816a41801c4254796b53361571d427990a6925acab66639f2a3c08d48cad24320b6fccf46a115745c09e4d13ba460d61eb4dff47d9b45f9f6f6b1919d095ddbf45ff3311e29d3a6b7d1bae0987e06388ad8862c6dd3bcf7ce88e88555ec444d281f2568850aa9ae369b9fc56b840fb3973baeea8acf934e26474b5ac2ed42233af643ccb917f1959ab4df65c74b7d6e8be81f347aea284f346f1a42ff63eba70525438a20e383106f6d3740be27d");
        let json = String::from(
            r#"{"parentHash":"0x00000000000000000000000000000000000000000000000067c1caf98fce5fed","miner":"0x00000000000000000000000019c0487fd937a206","stateRoot":"0x0000000000000000000000000000000000000000000000000bdbd3b900527e06","transactionsRoot":"0x00000000000000000000000000000000000000000000000023b81143fff0f675","receiptsRoot":"0x00000000000000000000000000000000000000000000000023aa2f717bea4bca","logsBloom":"0x21b4d0914ed15f3a50666926e3f411e7b379b9bb0de5bef605831e7131d4b5bf274481d47751be53fac7ed2bf3d84a2f778bb09776365fd95f29af413e1de49abb5e0a35d49f78ae9b38d46a0b0d27fa3d95d3e9022f8d28d7caff35a8fb180cc6fdf24d9ded8642b446807d5d80760fff524ea2143b4452bb7ff273ac75cc5405a5db65ff299db7816b26a063101dae8d46ccb8aaf2e9ffad86d46755f0070af3e12a9e8d787af7da95e587bb1f79816a41801c4254796b53361571d427990a6925acab66639f2a3c08d48cad24320b6fccf46a115745c09e4d13ba460d61eb4dff47d9b45f9f6f6b1919d095ddbf45ff3311e29d3a6b7d1bae0987e06388ad","number":"0x62c6dd3bcf7ce88e","gasUsed":"0x555ec444d281f256","timestamp":"0x50aa9ae369b9fc56","extraData":"0xfb3973baeea8acf934e26474b5ac2ed42233af643ccb917f1959ab4df65c74b7d6e8be81f347aea284f346f1a42ff63eba70525438a20e383106f6d3740be27d","hash":"0x916f9843415d9534935df2f1a623c7fcd057ac2269a4bc4900cbb55589bd1e58"}"#,
        );
        let expected = Header{parent_hash: H256::from_str("00000000000000000000000000000000000000000000000067c1caf98fce5fed").unwrap(), coinbase: H160::from_str("00000000000000000000000019c0487fd937a206").unwrap(), root: H256::from_str("0000000000000000000000000000000000000000000000000bdbd3b900527e06").unwrap(), tx_hash: H256::from_str("00000000000000000000000000000000000000000000000023b81143fff0f675").unwrap(), receipt_hash: H256::from_str("00000000000000000000000000000000000000000000000023aa2f717bea4bca").unwrap(), bloom: Bloom::from_str("21b4d0914ed15f3a50666926e3f411e7b379b9bb0de5bef605831e7131d4b5bf274481d47751be53fac7ed2bf3d84a2f778bb09776365fd95f29af413e1de49abb5e0a35d49f78ae9b38d46a0b0d27fa3d95d3e9022f8d28d7caff35a8fb180cc6fdf24d9ded8642b446807d5d80760fff524ea2143b4452bb7ff273ac75cc5405a5db65ff299db7816b26a063101dae8d46ccb8aaf2e9ffad86d46755f0070af3e12a9e8d787af7da95e587bb1f79816a41801c4254796b53361571d427990a6925acab66639f2a3c08d48cad24320b6fccf46a115745c09e4d13ba460d61eb4dff47d9b45f9f6f6b1919d095ddbf45ff3311e29d3a6b7d1bae0987e06388ad").unwrap(), number: U64::from(7117619510059526286 as u64), gas_used: U256::from(6151569940903227990 as u64), time: U256::from(5812628570597096534 as u64), extra: hex::decode("fb3973baeea8acf934e26474b5ac2ed42233af643ccb917f1959ab4df65c74b7d6e8be81f347aea284f346f1a42ff63eba70525438a20e383106f6d3740be27d").unwrap()};

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
