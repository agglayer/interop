use agglayer_interop_types::aggchain_proof::{
    AggchainData, AggchainProof, MultisigPayload, Proof, SP1StarkWithContext,
};
use bincode::Options as _;

use super::{sp1v4_bincode_options, Error};
use crate::v1::{self};

/// Maximum number of signers allowed in a multisig payload.
const MAX_SIGNERS: usize = 1024;

impl TryFrom<v1::AggchainProof> for AggchainProof {
    type Error = Error;

    fn try_from(value: v1::AggchainProof) -> Result<Self, Self::Error> {
        Ok(Self {
            proof: required_field!(value, proof),
            aggchain_params: required_field!(value, aggchain_params),
            public_values: value
                .context
                .get("public_values")
                .map(|b| {
                    std::panic::catch_unwind(|| bincode::deserialize(b))
                        .map_err(|_| {
                            Error::deserializing_aggchain_proof_public_values(Box::new(
                                bincode::ErrorKind::Custom(String::from("panic")),
                            ))
                        })?
                        .map_err(Error::deserializing_aggchain_proof_public_values)
                        .map(Box::new)
                })
                .transpose()?,
        })
    }
}

impl TryFrom<v1::aggchain_proof::Proof> for Proof {
    type Error = Error;

    fn try_from(value: v1::aggchain_proof::Proof) -> Result<Self, Self::Error> {
        Ok(match value {
            v1::aggchain_proof::Proof::Sp1Stark(proof) => proof.try_into()?,
        })
    }
}

impl TryFrom<v1::Sp1StarkProof> for Proof {
    type Error = Error;

    fn try_from(value: v1::Sp1StarkProof) -> Result<Self, Self::Error> {
        let v1::Sp1StarkProof {
            version,
            proof,
            vkey,
        } = value;

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
                    Error::deserializing_vkey(Box::new(bincode::ErrorKind::Custom(String::from(
                        "panic",
                    ))))
                })?
                .map_err(Error::deserializing_vkey)?,
            version,
        }))
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
            Some(v1::aggchain_data::Data::Generic(aggchain_proof)) => {
                let signature = aggchain_proof
                    .signature
                    .as_ref()
                    .map(|signature| {
                        (&*signature.value)
                            .try_into()
                            .map_err(Error::parsing_signature)
                    })
                    .transpose()?
                    .map(Box::new);

                let AggchainProof {
                    public_values,
                    aggchain_params,
                    proof,
                } = aggchain_proof.try_into()?;

                AggchainData::Generic {
                    public_values,
                    aggchain_params,
                    signature,
                    proof,
                }
            }
            Some(v1::aggchain_data::Data::Multisig(multisig)) => AggchainData::MultisigOnly {
                multisig: multisig.try_into()?,
            },
            Some(v1::aggchain_data::Data::MultisigAndAggchainProof(
                multisig_and_aggchain_proof,
            )) => AggchainData::MultisigAndAggchainProof {
                multisig: required_field!(multisig_and_aggchain_proof, multisig),
                aggchain_proof: required_field!(multisig_and_aggchain_proof, aggchain_proof),
            },
            None => return Err(Error::missing_field("data")),
        })
    }
}

impl TryFrom<v1::Multisig> for MultisigPayload {
    type Error = Error;

    fn try_from(multisig: v1::Multisig) -> Result<Self, Self::Error> {
        match multisig.data {
            Some(v1::multisig::Data::Ecdsa(v1::EcdsaMultisig { signatures })) => {
                if signatures.is_empty() {
                    return Err(Error::invalid_data(
                        "Multisig ECDSA doesn't have any signature".to_string(),
                    ));
                }

                // Find the maximum key to determine the vector size
                let required_len = signatures
                    .iter()
                    .map(|entry| entry.index + 1)
                    .max()
                    .unwrap_or(0);

                if required_len as usize > MAX_SIGNERS {
                    return Err(Error::invalid_data(format!(
                        "Multisig ECDSA has too many signers: {required_len} (max {MAX_SIGNERS})",
                    )));
                }

                // Create a vector filled with None, sized to accommodate the highest index
                let mut result: Vec<Option<_>> = vec![None; required_len as usize];

                // Fill in the signatures at their specified indices
                for entry in signatures {
                    let index = entry.index as usize;
                    if let Some(fixed_bytes) = entry.signature {
                        let signature = (&*fixed_bytes.value)
                            .try_into()
                            .map_err(Error::parsing_signature)?;

                        if result[index].is_some() {
                            return Err(Error::invalid_data(format!(
                                "Duplicate signature at index {index}",
                            )));
                        }

                        result[index] = Some(signature);
                    }
                }

                Ok(MultisigPayload(result))
            }
            None => Err(Error::missing_field("data")),
        }
    }
}
