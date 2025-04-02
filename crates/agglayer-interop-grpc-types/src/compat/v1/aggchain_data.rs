use std::collections::HashMap;

use agglayer_interop_types::aggchain_proof::{AggchainData, Proof, SP1StarkWithContext};
use bincode::Options as _;
use prost::bytes::Bytes;

use super::Error;
use crate::v1;

#[inline]
fn sp1v4_bincode_options() -> impl bincode::Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_fixint_encoding()
}

impl TryFrom<v1::AggchainData> for AggchainData {
    type Error = Error;

    fn try_from(value: v1::AggchainData) -> Result<Self, Self::Error> {
        Ok(match value.data {
            Some(v1::aggchain_data::Data::Signature(signature)) => AggchainData::ECDSA {
                signature: (&*signature.value)
                    .try_into()
                    .map_err(Error::parsing_signature)?,
            },
            Some(v1::aggchain_data::Data::Generic(proof)) => AggchainData::Generic {
                aggchain_params: required_field!(proof, aggchain_params),
                proof: match proof.proof {
                    Some(v1::aggchain_proof::Proof::Sp1Stark(v1::Sp1StarkProof {
                        version,
                        proof,
                        vkey,
                    })) => Proof::SP1Stark(SP1StarkWithContext {
                        proof: Box::new(
                            std::panic::catch_unwind(|| {
                                sp1v4_bincode_options().deserialize(&proof)
                            })
                            .map_err(|_| {
                                Error::deserializing_proof(Box::new(bincode::ErrorKind::Custom(
                                    String::from("panic"),
                                )))
                            })?
                            .map_err(Error::deserializing_proof)?,
                        ),
                        vkey: std::panic::catch_unwind(|| {
                            sp1v4_bincode_options().deserialize(&vkey)
                        })
                        .map_err(|_| {
                            Error::deserializing_proof(Box::new(bincode::ErrorKind::Custom(
                                String::from("panic"),
                            )))
                        })?
                        .map_err(Error::deserializing_vkey)?,
                        version,
                    }),
                    None => return Err(Error::missing_field("proof").inside_field("data")),
                },
            },
            None => return Err(Error::missing_field("data")),
        })
    }
}

impl TryFrom<AggchainData> for v1::AggchainData {
    type Error = Error;

    fn try_from(value: AggchainData) -> Result<Self, Self::Error> {
        Ok(v1::AggchainData {
            data: Some(match value {
                AggchainData::ECDSA { signature } => {
                    v1::aggchain_data::Data::Signature(v1::FixedBytes65 {
                        value: Bytes::copy_from_slice(&signature.as_bytes()),
                    })
                }
                AggchainData::Generic {
                    proof: Proof::SP1Stark(proof),
                    aggchain_params,
                } => v1::aggchain_data::Data::Generic(v1::AggchainProof {
                    context: HashMap::new(),
                    aggchain_params: Some(aggchain_params.into()),
                    proof: Some(v1::aggchain_proof::Proof::Sp1Stark(v1::Sp1StarkProof {
                        version: proof.version,
                        proof: sp1v4_bincode_options()
                            .serialize(&proof.proof)
                            .map_err(Error::serializing_proof)?
                            .into(),
                        vkey: sp1v4_bincode_options()
                            .serialize(&proof.vkey)
                            .map_err(Error::serializing_vkey)?
                            .into(),
                    })),
                }),
            }),
        })
    }
}
