use crate::errors::{Error};
use crate::proof::{Account, Proof};
use byte_slice_cast::AsByteSlice;
use core::borrow::Borrow;
use core::hash::Hasher as CoreHasher;
use ethereum_types::{Address, H256, U256};
use sha3::{Digest, Keccak256};
use std::convert::TryInto;
use trie_db::node::*;
use trie_db::triedbmut::ChildReference;
use trie_db::{Hasher, NibbleSlice, NodeCodec, Partial};

pub struct KHasher(Keccak256);
impl CoreHasher for KHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes)
    }
    fn finish(&self) -> u64 {
        u64::from_ne_bytes(
            self.0
                .clone()
                .finalize()
                .as_slice()
                .try_into()
                .expect("wrong size"),
        )
    }
}
impl Default for KHasher {
    fn default() -> Self {
        Self(Keccak256::new())
    }
}
impl Hasher for KHasher {
    type Out = H256;
    const LENGTH: usize = H256::len_bytes();
    type StdHasher = Self;
    fn hash(x: &[u8]) -> Self::Out {
        H256::from(Keccak256::digest(x).as_mut())
    }
}

pub struct RlpKeccaNodeCodec {}
impl NodeCodec for RlpKeccaNodeCodec {
    type HashOut = <KHasher as Hasher>::Out;
    type Error = rlp::DecoderError;
    const ESCAPE_HEADER: Option<u8> = None;
    fn hashed_null_node() -> Self::HashOut {
        KHasher::hash(&rlp::NULL_RLP)
    }
    fn decode_plan(_: &[u8]) -> Result<NodePlan, Self::Error> {
        unimplemented!()
    }
    fn is_empty_node(data: &[u8]) -> bool {
        rlp::Rlp::new(data).is_empty()
    }
    fn empty_node() -> &'static [u8] {
        &rlp::NULL_RLP
    }
    fn leaf_node(_partial: Partial<'_>, _value: Value<'_>) -> Vec<u8> {
        unimplemented!()
    }
    fn extension_node(
        _partial: impl Iterator<Item = u8>,
        _number_nibble: usize,
        _child_ref: ChildReference<Self::HashOut>,
    ) -> Vec<u8> {
        unimplemented!()
    }
    fn branch_node(
        _children: impl Iterator<Item = impl Borrow<Option<ChildReference<Self::HashOut>>>>,
        _value: Option<Value<'_>>,
    ) -> Vec<u8> {
        unimplemented!()
    }
    fn branch_node_nibbled(
        _partial: impl Iterator<Item = u8>,
        _number_nibble: usize,
        _children: impl Iterator<Item = impl Borrow<Option<ChildReference<Self::HashOut>>>>,
        _value: Option<Value<'_>>,
    ) -> Vec<u8> {
        unimplemented!()
    }

    fn decode(data: &[u8]) -> Result<Node, rlp::DecoderError> {
        const HASHLEN: usize = H256::len_bytes();
        let r = rlp::Rlp::new(data);
        match r.prototype()? {
            rlp::Prototype::List(2) => {
                //https://eth.wiki/fundamentals/patricia-tree#specification-compact-encoding-of-hex-sequence-with-optional-terminator
                let encoded_path = r.at(0)?.data()?;
                let value = r.at(1)?.data()?;
                let offset = if encoded_path[0] & 0x10 == 0x10 { 1 } else { 2 };
                let is_leaf = encoded_path[0] & 0x20 == 0x20;
                let nib = NibbleSlice::new_offset(encoded_path, offset);
                match (is_leaf, value.len()) {
                    (true, HASHLEN) => Ok(Node::Leaf(nib, Value::Node(value, None))),
                    (true, _) => Ok(Node::Leaf(nib, Value::Inline(value))),
                    (false, HASHLEN) => Ok(Node::Extension(nib, NodeHandle::Hash(value))),
                    (false, _) => Ok(Node::Extension(nib, NodeHandle::Inline(value))),
                }
            }
            rlp::Prototype::List(17) => {
                let mut nodes: [Option<NodeHandle>; 16] = [Some(NodeHandle::Inline(&[])); 16]; // Default::default();
                for i in 0..16 {
                    match r.at(i)?.prototype()? {
                        rlp::Prototype::Null | rlp::Prototype::Data(0) => {
                            nodes[i] = None;
                        }
                        rlp::Prototype::Data(HASHLEN) => {
                            let hash = r.at(i)?.data()?;
                            nodes[i] = Some(NodeHandle::Hash(hash));
                        }
                        rlp::Prototype::Data(_) => {
                            let data = r.at(i)?.data()?;
                            nodes[i] = Some(NodeHandle::Inline(data));
                        }
                        a => {
                            panic!("unexpected type {:?}", a);
                        }
                    }
                }
                let val = if r.at(16)?.is_empty() {
                    None
                } else {
                    Some(Value::Inline(r.at(16)?.data()?))
                };
                Ok(Node::Branch(nodes, val))
            }
            rlp::Prototype::Data(0) => Ok(Node::Empty),
            rlp::Prototype::Data(32) => {
                let slice = NibbleSlice::new(&[]);
                let data = r.data()?;
                Ok(Node::Leaf(slice, Value::Node(data, None)))
            }
            rlp::Prototype::Data(_) => {
                let slice = NibbleSlice::new(&[]);
                let data = r.data()?;
                Ok(Node::Leaf(slice, Value::Inline(data)))
            }
            _ => Err(Self::Error::Custom("Rlp is not valid.")),
        }
    }
}

pub struct CeloLayout {}
impl trie_db::TrieLayout for CeloLayout {
    type Codec = RlpKeccaNodeCodec;
    type Hash = KHasher;
    const USE_EXTENSION: bool = true;
    const ALLOW_EMPTY: bool = true;
    const MAX_INLINE_VALUE: Option<u32> = Some(H256::len_bytes() as u32);
}

pub fn verify(
    proof: &Proof,
    root: H256,
    address: Address,
    expected_key: U256,
    expected_value: Option<U256>,
) -> Result<(), Error> {
    if let Some(storage_proof) = proof.storage_proof.first() {
        if storage_proof.key != expected_key {
            return Err(Error::StorageProofKeyNotMatching {
                current: storage_proof.key,
                expected: expected_key,
            });
        }
        if storage_proof.value != expected_value.unwrap_or_default() {
            return Err(Error::StorageProofValueNotMatching {
                current: storage_proof.value,
                expected: expected_value.unwrap_or_default(),
            });
        }
        let value = expected_value.map(|v| AsByteSlice::as_byte_slice(&v).to_vec());
        trie_db::proof::eip1186::verify_proof::<CeloLayout>(
            &proof.storage_hash,
            &storage_proof.proof,
            AsByteSlice::as_byte_slice(&expected_key),
            value.as_deref(),
        )
        .map_err(|e| Error::ProofVerification {
            error: format!("{:?}", e),
        })?;
        let account = Account {
            balance: proof.balance,
            code_hash: proof.code_hash,
            nonce: proof.nonce,
            storage_hash: proof.storage_hash,
        };
        let key = Keccak256::digest(address.as_ref());
        trie_db::proof::eip1186::verify_proof::<CeloLayout>(
            &root,
            &proof.account_proof,
            &key,
            Some(&rlp::encode(&account)),
        )
        .map_err(|e| Error::ProofVerification {
            error: format!("{:?}", e),
        })?;
        Ok(())
    } else {
        Err(Error::StorageProofKeyNotPresent)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::proof::Account;
    use crate::test_proofs;
    use byte_slice_cast::AsByteSlice;
    use ethereum_types::*;
    use std::str::FromStr;

    #[test]
    fn hash_null_node() {
        let t1 = String::default();
        let t2 = rlp::encode(&t1);
        let t3 = KHasher::hash(&t2);
        let t4 = KHasher::hash(&rlp::NULL_RLP);
        assert_eq!(t3, t4);
    }

    #[test]
    fn test_decode_leaf() {}

    #[test]
    fn test_decode_extension() {
        let extension = hex::decode(
            "e4850001020304ddc882350684636f696e8080808080808080808080808080808476657262",
        )
        .unwrap();
        let res = RlpKeccaNodeCodec::decode(&extension);
        assert!(res.is_ok());
        assert!(matches!(res, Ok(Node::Extension(_, _))));
    }

    #[test]
    fn test_decoder_1() {
        let (proof, _, _) = test_proofs::proof1();
        for node in proof.account_proof {
            let res = RlpKeccaNodeCodec::decode(&node);
            assert!(res.is_ok());
        }

        for storage_proof in proof.storage_proof {
            for proof in storage_proof.proof {
                let res = RlpKeccaNodeCodec::decode(&proof);
                assert!(res.is_ok());
            }
        }
    }

    #[test]
    fn test_decoder_2() {
        let (proof, _, _) = test_proofs::proof2();
        for node in proof.account_proof {
            let res = RlpKeccaNodeCodec::decode(&node);
            assert!(res.is_ok());
        }

        for storage_proof in proof.storage_proof {
            for proof in storage_proof.proof {
                let res = RlpKeccaNodeCodec::decode(&proof);
                assert!(res.is_ok());
            }
        }
    }

    #[test]
    fn test_decoder_3() {
        let (proof, _, _) = test_proofs::proof3();
        for node in proof.account_proof {
            let res = RlpKeccaNodeCodec::decode(&node);
            assert!(res.is_ok());
        }

        for storage_proof in proof.storage_proof {
            for proof in storage_proof.proof {
                let res = RlpKeccaNodeCodec::decode(&proof);
                assert!(res.is_ok());
            }
        }
    }

    #[test]
    fn celo_layout_account_proof_1() {
        let (proof, address, root) = test_proofs::proof1();
        let account = Account {
            balance: proof.balance,
            code_hash: proof.code_hash,
            nonce: proof.nonce,
            storage_hash: proof.storage_hash,
        };
        let key = Keccak256::digest(address.as_ref());
        let res = trie_db::proof::eip1186::verify_proof::<CeloLayout>(
            &root,
            &proof.account_proof,
            &key,
            Some(&rlp::encode(&account)),
        );
        assert!(res.is_ok());
    }

    #[test]
    fn celo_layout_storage_proof_1() {
        let (proof, address, root) = test_proofs::proof1();
        for storage_proof in proof.storage_proof {
            let key = Keccak256::digest(AsByteSlice::as_byte_slice(&storage_proof.key));
            let value = &rlp::encode(&storage_proof.value);
            let res = trie_db::proof::eip1186::verify_proof::<CeloLayout>(
                &proof.storage_hash,
                &storage_proof.proof,
                &key,
                Some(&value),
            );
            assert!(res.is_ok());
        }
    }

    #[test]
    fn celo_layout_account_proof_2() {
        let (proof, address, root) = test_proofs::proof2();
        let account = Account {
            balance: proof.balance,
            code_hash: proof.code_hash,
            nonce: proof.nonce,
            storage_hash: proof.storage_hash,
        };
        let key = Keccak256::digest(address.as_ref());
        let res = trie_db::proof::eip1186::verify_proof::<CeloLayout>(
            &root,
            &proof.account_proof,
            &key,
            Some(&rlp::encode(&account)),
        );
        assert!(res.is_ok());
    }

    #[test]
    fn celo_layout_account_proof_3() {
        let (proof, address, root) = test_proofs::proof3();
        let account = Account {
            balance: proof.balance,
            code_hash: proof.code_hash,
            nonce: proof.nonce,
            storage_hash: proof.storage_hash,
        };
        let key = Keccak256::digest(address.as_ref());
        let res = trie_db::proof::eip1186::verify_proof::<CeloLayout>(
            &root,
            &proof.account_proof,
            &key,
            Some(&rlp::encode(&account)),
        );
        assert!(res.is_ok());
    }

    #[test]
    fn celo_layout_storage_proof_3() {
        let (proof, address, root) = test_proofs::proof3();
        for storage_proof in proof.storage_proof {
            let key = Keccak256::digest(AsByteSlice::as_byte_slice(&storage_proof.key));
            let value = &rlp::encode(&storage_proof.value);
            let res = trie_db::proof::eip1186::verify_proof::<CeloLayout>(
                &proof.storage_hash,
                &storage_proof.proof,
                &key,
                Some(&value),
            );
            assert!(res.is_ok());
        }
    }
}
