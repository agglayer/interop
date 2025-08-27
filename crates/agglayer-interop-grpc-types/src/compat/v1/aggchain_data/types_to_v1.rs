use std::collections::HashMap;

use agglayer_interop_types::aggchain_proof::{
    AggchainData, AggchainProof, MultisigPayload, Proof, SP1StarkWithContext,
};
use bincode::Options as _;
use prost::bytes::Bytes;

use super::{sp1v4_bincode_options, Error};
use crate::v1::{self};

impl TryFrom<AggchainProof> for v1::AggchainProof {
    type Error = Error;

    fn try_from(value: AggchainProof) -> Result<Self, Self::Error> {
        Ok(v1::AggchainProof {
            proof: Some(value.proof.try_into()?),
            aggchain_params: Some(value.aggchain_params.into()),
            signature: None,
            context: match value.public_values {
                Some(public_values) => HashMap::from([(
                    "public_values".to_owned(),
                    Bytes::from(
                        bincode::serialize(&*public_values).map_err(Error::serializing_context)?,
                    ),
                )]),
                None => HashMap::new(),
            },
        })
    }
}

impl TryFrom<Proof> for v1::aggchain_proof::Proof {
    type Error = Error;

    fn try_from(value: Proof) -> Result<Self, Self::Error> {
        match value {
            Proof::SP1Stark(SP1StarkWithContext {
                proof,
                vkey,
                version,
            }) => Ok(v1::aggchain_proof::Proof::Sp1Stark(v1::Sp1StarkProof {
                proof: sp1v4_bincode_options()
                    .serialize(&proof)
                    .map_err(|_| {
                        Error::serializing_proof(Box::new(bincode::ErrorKind::Custom(
                            String::from("panic"),
                        )))
                    })?
                    .into(),
                vkey: sp1v4_bincode_options()
                    .serialize(&vkey)
                    .map_err(|_| {
                        Error::serializing_vkey(Box::new(bincode::ErrorKind::Custom(String::from(
                            "panic",
                        ))))
                    })?
                    .into(),
                version,
            })),
        }
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
                                .map_err(Error::serializing_context)?,
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
                    aggchain_proof,
                } => v1::aggchain_data::Data::MultisigAndAggchainProof(
                    v1::AggchainProofWithMultisig {
                        multisig: Some(multisig.into()),
                        aggchain_proof: Some(aggchain_proof.try_into()?),
                    },
                ),
            }),
        })
    }
}

impl From<MultisigPayload> for v1::Multisig {
    fn from(multisig: MultisigPayload) -> Self {
        Self {
            data: Some(v1::multisig::Data::Ecdsa(v1::EcdsaMultisig {
                signatures: multisig
                    .0
                    .iter()
                    .enumerate()
                    .map(|(key, sig)| v1::ecdsa_multisig::EcdsaMultisigEntry {
                        key: key.try_into().unwrap_or(u64::MAX),

                        value: sig.map(|sig| v1::FixedBytes65 {
                            value: Bytes::copy_from_slice(&sig.as_bytes()),
                        }),
                    })
                    .collect(),
            })),
        }
    }
}
