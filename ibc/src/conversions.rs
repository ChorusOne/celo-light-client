use crate::ConnectionEnd;

pub use ibc_proto::ibc::core::client::v1::Height as IBCHeight;
pub use ibc_proto::ibc::core::commitment::v1::{MerkleRoot as IBCMerkleRoot, MerklePrefix as IBCMerklePrefix};
pub use ibc_proto::ibc::lightclients::wasm::v1::ClientState as IBCClientState;
pub use ibc_proto::ibc::lightclients::wasm::v1::ConsensusState as IBCConsensusState;
pub use ibc_proto::ibc::lightclients::wasm::v1::Header as IBCHeader;
pub use ibc_proto::ibc::lightclients::wasm::v1::Misbehaviour as IBCMisbehaviour;
pub use ibc_proto::ics23::InnerSpec as ICSInnerSpec;
pub use ibc_proto::ics23::LeafOp as ICSLeafOp;
pub use ibc_proto::ics23::ProofSpec as ICSProofSpec;


/// ConnectionEnd <-> IBCConnectionEnd
impl TryFrom<IBCConnectionEnd> for ConnectionEnd {
    type Error = Error;
    fn try_from(ibc: IBCConnectionEnd) -> Result<Self, Self::Error> {
        let counter = if let Some(cp) = ibc.counterparty {
            Some(Counterparty::try_from(cp)?)
        } else {
            None
        };
        let s = Self {
            client_id: ibc.client_id,
            versions: ibc.versions.into_iter().map(Version::from).collect(),
            state: ibc.state,
            counterparty: counter.ok_or_else(|| Kind::MissingField {
                struct_name: String::from("ConnectionEnd"),
                field_name: String::from("Counterparty"),
            })?,
            delay_period: ibc.delay_period,
        };
        Ok(s)
    }
}

/// Counterparty <-> IBCCounterparty
impl TryFrom<IBCCounterparty> for Counterparty {
    type Error = Error;
    fn try_from(c: IBCCounterparty) -> Result<Self, Self::Error> {
        let s = Self {
            client_id: c.client_id,
            connection_id: c.connection_id,
            prefix: c
                .prefix
                .map(MerklePrefix::from)
                .ok_or_else(|| Kind::MissingField {
                    struct_name: String::from("Counterparty"),
                    field_name: String::from("prefix"),
                })?,
        };
        Ok(s)
    }
}
impl TryFrom<Counterparty> for IBCCounterparty {
    type Error = base64::DecodeError;
    fn try_from(c: Counterparty) -> Result<Self, Self::Error> {
        let s = Self {
            client_id: c.client_id,
            connection_id: c.connection_id,
            prefix: Some(IBCMerklePrefix::try_from(c.prefix)?),
        };
        Ok(s)
    }
}


/// Conversion Version <-> IBCVersion
impl From<IBCVersion> for Version {
    fn from(v: IBCVersion) -> Self {
        Self {
            identifier: v.identifier,
            features: v.features,
        }
    }
}
impl From<Version> for IBCVersion {
    fn from(v: Version) -> Self {
        Self {
            identifier: v.identifier,
            features: v.features,
        }
    }
}

///
impl From<IBCMerklePrefix> for MerklePrefix {
    fn from(ibc: IBCMerklePrefix) -> Self {
        Self {
            key_prefix: base64::encode(ibc.key_prefix),
        }
    }
}
impl TryFrom<MerklePrefix> for IBCMerklePrefix {
    type Error = base64::DecodeError;
    fn try_from(m: MerklePrefix) -> Result<Self, Self::Error> {
        let s = Self {
            key_prefix: base64::decode(m.key_prefix)?,
        };
        Ok(s)
    }
}

/// 
impl From<IBCMerkleRoot> for MerkleRoot {
    fn from(ibc: IBCMerkleRoot) -> Self {
        Self {
            hash: base64::encode(ibc.hash),
        }
    }
}
impl TryFrom<MerkleRoot> for IBCMerkleRoot {
    type Error = base64::DecodeError;
    fn try_from(m: MerkleRoot) -> Result<Self, Self::Error> {
        let s = Self {
            hash: base64::decode(m.hash)?,
        };
        Ok(s)
    }
}

///
impl From<ICSProofSpec> for ProofSpec {
    fn from(ics: ICSProofSpec) -> Self {
        Self {
            leaf_spec: ics.leaf_spec.map(LeafOp::from),
            inner_spec: ics.inner_spec.map(InnerSpec::from),
            max_depth: ics.max_depth,
            min_depth: ics.min_depth,
        }
    }
}
impl TryFrom<ProofSpec> for ICSProofSpec {
    type Error = base64::DecodeError;
    fn try_from(p: ProofSpec) -> Result<Self, Self::Error> {
        let inner = if let Some(i) = p.inner_spec {
            Some(ICSInnerSpec::try_from(i)?)
        } else {
            None
        };
        let leaf = if let Some(l) = p.leaf_spec {
            Some(ICSLeafOp::try_from(l)?)
        } else {
            None
        };
        let s = Self {
            leaf_spec: leaf,
            inner_spec: inner,
            max_depth: p.max_depth,
            min_depth: p.min_depth,
        };
        Ok(s)
    }
}

///
impl From<ICSInnerSpec> for InnerSpec {
    fn from(ics: ICSInnerSpec) -> Self {
        Self {
            child_order: ics.child_order,
            child_size: ics.child_size,
            min_prefix_length: ics.min_prefix_length,
            max_prefix_length: ics.max_prefix_length,
            empty_child: base64::encode(ics.empty_child),
            hash: ics.hash,
        }
    }
}
impl TryFrom<InnerSpec> for ICSInnerSpec {
    type Error = base64::DecodeError;
    fn try_from(i: InnerSpec) -> Result<Self, Self::Error> {
        let s = Self {
            child_order: i.child_order,
            child_size: i.child_size,
            min_prefix_length: i.min_prefix_length,
            max_prefix_length: i.max_prefix_length,
            empty_child: base64::decode(i.empty_child)?,
            hash: i.hash,
        };
        Ok(s)
    }
}

///
impl From<ICSLeafOp> for LeafOp {
    fn from(ics: ICSLeafOp) -> Self {
        Self {
            hash: ics.hash,
            prehash_key: ics.prehash_key,
            prehash_value: ics.prehash_value,
            length: ics.length,
            prefix: base64::encode(ics.prefix),
        }
    }
}
impl TryFrom<LeafOp> for ICSLeafOp {
    type Error = base64::DecodeError;
    fn try_from(l: LeafOp) -> Result<Self, Self::Error> {
        let s = Self {
            hash: l.hash,
            prehash_key: l.prehash_key,
            prehash_value: l.prehash_value,
            length: l.length,
            prefix: base64::decode(l.prefix)?,
        };
        Ok(s)
    }
}

///
impl TryFrom<IBCConsensusState> for ConsensusState {
    type Error = Error;
    fn try_from(ibc: IBCConsensusState) -> Result<Self, Self::Error> {
        let root = ibc.root.ok_or_else(|| Kind::MissingField {
            struct_name: String::from("IBCConsensusState"),
            field_name: String::from("root"),
        })?;
        let s = Self {
            data: base64::encode(ibc.data),
            code_id: base64::encode(ibc.code_id),
            timestamp: ibc.timestamp,
            root: MerkleRoot::from(root),
        };
        Ok(s)
    }
}
impl TryFrom<ConsensusState> for IBCConsensusState {
    type Error = base64::DecodeError;
    fn try_from(cs: ConsensusState) -> Result<Self, Self::Error> {
        let s = Self {
            data: base64::decode(&cs.data)?,
            code_id: base64::decode(&cs.code_id)?,
            timestamp: cs.timestamp,
            root: Some(IBCMerkleRoot::try_from(cs.root)?),
        };
        Ok(s)
    }
}

///
impl TryFrom<IBCClientState> for ClientState {
    type Error = Error;
    fn try_from(ibc: IBCClientState) -> Result<Self, Self::Error> {
        let specs = ibc.proof_specs.into_iter().map(ProofSpec::from).collect();
        let s = Self {
            data: base64::encode(ibc.data),
            code_id: base64::encode(ibc.code_id),
            latest_height: ibc.latest_height.ok_or_else(|| Kind::MissingField {
                struct_name: String::from("IBCClientState"),
                field_name: String::from("latest_height"),
            })?,
            proof_specs: specs,
            frozen_height: None,
        };
        Ok(s)
    }
}
impl TryFrom<ClientState> for IBCClientState {
    type Error = base64::DecodeError;
    fn try_from(cs: ClientState) -> Result<Self, Self::Error> {
        let specs: Vec<ICSProofSpec> = cs
            .proof_specs
            .into_iter()
            .map(ICSProofSpec::try_from)
            .collect::<Result<_, _>>()?;
        let s = Self {
            data: base64::decode(&cs.data)?,
            code_id: base64::decode(&cs.code_id)?,
            latest_height: Some(cs.latest_height),
            proof_specs: specs,
        };
        Ok(s)
    }
}

///
impl TryFrom<IBCChannel> for Channel {
    type Error = Error;
    fn try_from(c: IBCChannel) -> Result<Self, Self::Error> {
        let s = Self {
            state: c.state,
            ordering: c.ordering,
            counterparty: c.counterparty.map(Counterparty::from).ok_or_else(|| {
                Kind::MissingField {
                    struct_name: String::from("Channel"),
                    field_name: String::from("counterparty"),
                }
            })?,
            connection_hops: c.connection_hops,
            version: c.version,
        };
        Ok(s)
    }
}
impl From<Channel> for IBCChannel {
    fn from(c: Channel) -> Self {
        Self {
            state: c.state,
            ordering: c.ordering,
            counterparty: Some(IBCCounterparty::from(c.counterparty)),
            connection_hops: c.connection_hops,
            version: c.version,
        }
    }
}

///
impl From<IBCCounterparty> for Counterparty {
    fn from(c: IBCCounterparty) -> Self {
        Self {
            port_id: c.port_id,
            channel_id: c.channel_id,
        }
    }
}
impl From<Counterparty> for IBCCounterparty {
    fn from(c: Counterparty) -> Self {
        Self {
            port_id: c.port_id,
            channel_id: c.channel_id,
        }
    }
}

///
impl TryFrom<IBCMisbehaviour> for Misbehaviour {
    type Error = Error;
    fn try_from(ibc: IBCMisbehaviour) -> Result<Self, Self::Error> {
        let head1 = ibc.header_1.ok_or_else(|| Kind::MissingField {
            struct_name: String::from("IBCMisbehaviour"),
            field_name: String::from("header_1"),
        })?;
        let head2 = ibc.header_2.ok_or_else(|| Kind::MissingField {
            struct_name: String::from("IBCMisbehaviour"),
            field_name: String::from("header_1"),
        })?;

        let s = Self {
            client_id: ibc.client_id,
            header_1: Header::try_from(head1)?,
            header_2: Header::try_from(head2)?,
        };
        Ok(s)
    }
}
impl TryFrom<Misbehaviour> for IBCMisbehaviour {
    type Error = base64::DecodeError;
    fn try_from(mis: Misbehaviour) -> Result<Self, Self::Error> {
        let s = Self {
            client_id: mis.client_id,
            header_1: Some(mis.header_1.try_into()?),
            header_2: Some(mis.header_2.try_into()?),
        };
        Ok(s)
    }
}

///
impl TryFrom<IBCHeader> for Header {
    type Error = Error;
    fn try_from(ibc: IBCHeader) -> Result<Self, Self::Error> {
        let s = Self {
            data: base64::encode(ibc.data),
            height: ibc.height.ok_or_else(|| Kind::MissingField {
                struct_name: String::from("IBCHeader"),
                field_name: String::from("height"),
            })?,
        };
        Ok(s)
    }
}
impl TryFrom<Header> for IBCHeader {
    type Error = base64::DecodeError;
    fn try_from(h: Header) -> Result<Self, Self::Error> {
        let s = Self {
            data: base64::decode(&h.data)?,
            height: Some(h.height),
        };
        Ok(s)
    }
}
