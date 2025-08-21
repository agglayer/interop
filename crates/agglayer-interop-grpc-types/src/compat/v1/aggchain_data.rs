use std::collections::HashMap;

use agglayer_interop_types::aggchain_proof::{
    AggchainData, AggchainProof, MultisigPayload, Proof, SP1StarkWithContext,
};
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

impl TryFrom<v1::Sp1StarkProof> for Proof {
    type Error = Error;

    fn try_from(
        v1::Sp1StarkProof {
            version,
            proof,
            vkey,
        }: v1::Sp1StarkProof,
    ) -> Result<Self, Self::Error> {
        Ok(Proof::SP1Stark(SP1StarkWithContext {
            proof: Box::new(
                std::panic::catch_unwind(|| sp1v4_bincode_options().deserialize(&proof))
                    .map_err(|_| {
                        Error::deserializing_proof(Box::new(bincode::ErrorKind::Custom(
                            String::from("panic"),
                        )))
                    })?
                    .map_err(Error::deserializing_proof)?,
            ),
            vkey: std::panic::catch_unwind(|| sp1v4_bincode_options().deserialize(&vkey))
                .map_err(|_| {
                    Error::deserializing_proof(Box::new(bincode::ErrorKind::Custom(String::from(
                        "panic",
                    ))))
                })?
                .map_err(Error::deserializing_vkey)?,
            version,
        }))
    }
}

impl TryFrom<v1::Multisig> for MultisigPayload {
    type Error = Error;

    fn try_from(v1::Multisig { signatures }: v1::Multisig) -> Result<Self, Self::Error> {
        Ok(signatures
            .iter()
            .map(|signature| {
                (&*signature.value)
                    .try_into()
                    .map_err(Error::parsing_signature)
            })
            .collect())
    }
}

impl TryFrom<v1::AggchainProofWithMultisig> for AggchainData {
    type Error = Error;

    fn try_from(
        v1::AggchainProofWithMultisig {
            multisig,
            aggchain_params,
            context,
            proof,
        }: v1::AggchainProofWithMultisig,
    ) -> Result<Self, Self::Error> {
        Ok(AggchainData::MultisigAndAggchainProof {
            multisig: multisig.try_into()?,
            aggchain_proof: AggchainProof {
                proof: match proof {
                    Some(proof) => proof.try_into()?,
                    None => return Err(Error::missing_field("proof").inside_field("data")),
                },
                public_values: context
                    .get("public_values")
                    .map(|b| bincode::deserialize(b).map(Box::new))
                    .transpose()
                    .map_err(Error::deserializing_aggchain_proof_public_values)?,
                aggchain_params: required_field!(proof, aggchain_params),
            },
        })
    }
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
                public_values: proof
                    .context
                    .get("public_values")
                    .map(|b| bincode::deserialize(b).map(Box::new))
                    .transpose()
                    .map_err(Error::deserializing_aggchain_proof_public_values)?,
                aggchain_params: required_field!(proof, aggchain_params),
                signature: proof
                    .signature
                    .map(|signature| {
                        (&*signature.value)
                            .try_into()
                            .map_err(Error::parsing_signature)
                    })
                    .transpose()?
                    .map(Box::new),
                proof: match proof.proof {
                    Some(proof) => proof.try_into()?,
                    None => return Err(Error::missing_field("proof").inside_field("data")),
                },
            },
            Some(v1::aggchain_data::Data::Multisig(multisig)) => multisig.try_into()?,
            Some(v1::aggchain_data::Data::MultisigAndAggchainProof(
                multisig_and_aggchain_proof,
            )) => multisig_and_aggchain_proof.try_into(),
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
                    signature,
                    aggchain_params,
                    public_values,
                } => v1::aggchain_data::Data::Generic(v1::AggchainProof {
                    context: HashMap::from([(
                        "public_values".to_owned(),
                        Bytes::from(
                            bincode::serialize(&public_values)
                                .unwrap_or_else(|_| b"bincode serialization failed".to_vec()),
                        ),
                    )]),
                    aggchain_params: Some(aggchain_params.into()),
                    signature: signature.map(|signature| v1::FixedBytes65 {
                        value: Bytes::copy_from_slice(&signature.as_bytes()),
                    }),
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
                AggchainData::MultisigOnly(multisig) => {
                    v1::aggchain_data::Data::Multisig(multisig.into())
                }
                AggchainData::MultisigAndAggchainProof {
                    multisig,
                    aggchain_proof:
                        AggchainProof {
                            proof: Proof::SP1Stark(proof),
                            aggchain_params,
                            public_values,
                        },
                } => v1::aggchain_data::Data::MultisigAndAggchainProof(
                    v1::AggchainProofWithMultisig {
                        multisig: multisig.into(),
                        context: HashMap::from([(
                            "public_values".to_owned(),
                            Bytes::from(
                                bincode::serialize(&public_values)
                                    .unwrap_or_else(|_| b"bincode serialization failed".to_vec()),
                            ),
                        )]),
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
                    },
                ),
            }),
        })
    }
}

impl From<MultisigPayload> for v1::Multisig {
    fn from(multisig: MultisigPayload) -> Self {
        Self {
            signatures: multisig
                .iter()
                .map(|sig| v1::FixedBytes65 {
                    value: Bytes::copy_from_slice(&sig.as_bytes()),
                })
                .collect(),
        }
    }
}
