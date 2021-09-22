use celo_types::Header;
use std::convert::{From, TryFrom};
use web3::block_on;
use web3::transports::Http;
use web3::types::{BlockId, BlockNumber, U64};

pub struct Relayer {
    web3: web3::Web3<Http>,
}

impl Relayer {
    pub fn new(uri: String) -> Self {
        let transport = web3::transports::Http::new(&uri).expect("invalid url?");
        let web3 = web3::Web3::new(transport);
        Self { web3 }
    }

    pub fn get_block_header_by_number(&self, number: BlockNumber) -> Header {
        let block_id = BlockId::Number(number);
        let blk = block_on(self.web3.eth().block(block_id))
            .expect("couldn't find block number")
            .expect("block does not exist");
        Header::try_from(blk).expect("parsing the block to celo header")
    }

}
