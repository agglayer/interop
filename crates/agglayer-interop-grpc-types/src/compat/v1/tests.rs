use agglayer_interop_types::{
    aggchain_proof::{
        AggchainData, AggchainProof, AggchainProofPublicValues, MultisigPayload, Proof,
        SP1StarkWithContext,
    },
    primitives::SignatureError,
    Address, BridgeExit, ClaimFromMainnet, ClaimFromRollup, Digest, GlobalIndex,
    ImportedBridgeExit, L1InfoTreeLeaf, L1InfoTreeLeafInner, MerkleProof, NetworkId, Signature,
    TokenInfo, U256,
};
use prost::Message;

use super::Error;
use crate::v1;

#[rstest::rstest]
#[case::error("no_proof", Error::missing_field("proof"))]
#[case::error("bad_data", Error::invalid_data("invalid value".to_owned()))]
#[case::error("bad_data_in_field", Error::invalid_data("invalid value".to_owned()).inside_field("value"))]
#[case::error("bad_data_in_nested", Error::invalid_data("invalid value".to_owned()).inside_field("value").inside_field("data"))]
#[case::error("failed_ser", Error::serializing_proof(Box::new(bincode::ErrorKind::Custom("failed".to_owned()))))]
#[case::error("failed_deser", Error::deserializing_proof(Box::new(bincode::ErrorKind::Custom("failed".to_owned()))))]
#[case::error("bad_sig", Error::parsing_signature(SignatureError::InvalidParity(5)))]
#[case::error("bad_sig_in_nested", Error::parsing_signature(SignatureError::InvalidParity(5)).inside_field("signature").inside_field("data"))]
fn error_messages(#[case] name: &str, #[case] error: Error) {
    insta::assert_snapshot!(format!("{name}/display"), error);
    insta::assert_debug_snapshot!(format!("{name}/kind"), error.kind());
    insta::assert_snapshot!(
        format!("{name}/debug"),
        format!("{:?}", eyre::Error::from(error))
    );
}

macro_rules! make_parser_fuzzers {
    ($test:ident, $proto:ty, $type:ty) => {
        #[test]
        fn $test() {
            bolero::check!().for_each(|bytes| {
                if let Ok(proto) = <$proto>::decode(bytes) {
                    let _ = <$type>::try_from(proto);
                };
            })
        }
    };
}

make_parser_fuzzers!(fuzz_parser_address, v1::FixedBytes20, Address);
make_parser_fuzzers!(fuzz_parser_aggchain_data, v1::AggchainData, AggchainData);
make_parser_fuzzers!(fuzz_parser_bridge_exit, v1::BridgeExit, BridgeExit);
make_parser_fuzzers!(
    fuzz_parser_claim_from_mainnet,
    v1::ClaimFromMainnet,
    ClaimFromMainnet
);
make_parser_fuzzers!(
    fuzz_parser_claim_from_rollup,
    v1::ClaimFromRollup,
    ClaimFromRollup
);
make_parser_fuzzers!(fuzz_parser_digest, v1::FixedBytes32, Digest);
make_parser_fuzzers!(fuzz_parser_global_index, v1::FixedBytes32, GlobalIndex);
make_parser_fuzzers!(
    fuzz_parser_imported_bridge_exit,
    v1::ImportedBridgeExit,
    ImportedBridgeExit
);
make_parser_fuzzers!(
    fuzz_parser_l1_info_tree_leaf_with_context,
    v1::L1InfoTreeLeafWithContext,
    L1InfoTreeLeaf
);
make_parser_fuzzers!(
    fuzz_parser_l1_info_tree_leaf_inner,
    v1::L1InfoTreeLeaf,
    L1InfoTreeLeafInner
);
make_parser_fuzzers!(fuzz_parser_merkle_proof, v1::MerkleProof, MerkleProof);
make_parser_fuzzers!(fuzz_parser_token_info, v1::TokenInfo, TokenInfo);
make_parser_fuzzers!(fuzz_parser_u256, v1::FixedBytes32, U256);

macro_rules! make_round_trip_fuzzers {
    ($test:ident, $proto:ty, $type:ty) => {
        #[test]
        fn $test() {
            bolero::check!()
                .with_arbitrary::<$type>()
                .for_each(|input: &$type| {
                    let proto: $proto = input.clone().into();
                    let output = <$type>::try_from(proto).unwrap();
                    assert_eq!(input, &output);
                })
        }
    };
}

make_round_trip_fuzzers!(fuzz_round_trip_address, v1::FixedBytes20, Address);

fn sample_public_values() -> AggchainProofPublicValues {
    AggchainProofPublicValues {
        prev_local_exit_root: Digest([0x11; 32]),
        new_local_exit_root: Digest([0x22; 32]),
        l1_info_root: Digest([0x33; 32]),
        origin_network: NetworkId::new(0x00112233),
        commit_imported_bridge_exits: Digest([0x44; 32]),
        aggchain_params: Digest([0x55; 32]),
    }
}

fn sample_proof() -> Proof {
    Proof::SP1Stark(SP1StarkWithContext {
        version: "v6.0.0".to_owned(),
        proof: vec![0x06, 0x00, 0x00, 0x01],
        vkey: vec![0xb6, 0x00, 0x02],
    })
}

fn sample_signature() -> Signature {
    Signature::new(U256::from(1_u64), U256::from(2_u64), true)
}

fn sample_generic(public_values: Option<Box<AggchainProofPublicValues>>) -> AggchainData {
    AggchainData::Generic {
        proof: sample_proof(),
        aggchain_params: Digest([0x77; 32]),
        signature: None,
        public_values,
    }
}

fn sample_aggchain_proof(public_values: Option<Box<AggchainProofPublicValues>>) -> AggchainProof {
    AggchainProof {
        proof: sample_proof(),
        aggchain_params: Digest([0x88; 32]),
        public_values,
    }
}

fn sample_multisig_and_aggchain_proof(
    public_values: Option<Box<AggchainProofPublicValues>>,
) -> AggchainData {
    AggchainData::MultisigAndAggchainProof {
        multisig: MultisigPayload(vec![Some(sample_signature())]),
        aggchain_proof: sample_aggchain_proof(public_values),
    }
}

fn bare_public_values_expected_bytes() -> Vec<u8> {
    [
        [0x11; 32].as_slice(),
        [0x22; 32].as_slice(),
        [0x33; 32].as_slice(),
        &[0x33, 0x22, 0x11, 0x00],
        [0x44; 32].as_slice(),
        [0x55; 32].as_slice(),
    ]
    .concat()
}

fn generic_some_public_values_expected_bytes() -> Vec<u8> {
    let mut v = Vec::with_capacity(1 + 164);
    v.push(0x01);
    v.extend_from_slice(&bare_public_values_expected_bytes());
    v
}

fn generic_none_public_values_expected_bytes() -> Vec<u8> {
    vec![0x00]
}

#[test]
fn aggchain_proof_v1_wire_uses_sp1_compatible_public_values_bytes() {
    let input = sample_aggchain_proof(Some(Box::new(sample_public_values())));

    let proto: v1::AggchainProof = input.try_into().unwrap();
    let encoded = proto
        .context
        .get("public_values")
        .expect("AggchainProof v1 wire format includes public_values when Some");

    assert_eq!(
        encoded.as_ref(),
        bare_public_values_expected_bytes().as_slice()
    );
}

#[test]
fn aggchain_proof_v1_wire_omits_public_values_when_none() {
    let input = sample_aggchain_proof(None);

    let proto: v1::AggchainProof = input.try_into().unwrap();

    assert!(!proto.context.contains_key("public_values"));
}

#[rstest::rstest]
#[case::some(
    Some(Box::new(sample_public_values())),
    generic_some_public_values_expected_bytes()
)]
#[case::none(None, generic_none_public_values_expected_bytes())]
fn generic_v1_wire_uses_sp1_compatible_public_values_bytes(
    #[case] public_values: Option<Box<AggchainProofPublicValues>>,
    #[case] expected_bytes: Vec<u8>,
) {
    let input = sample_generic(public_values);

    let proto: v1::AggchainData = input.try_into().unwrap();
    let v1::aggchain_data::Data::Generic(proto_generic) = proto.data.as_ref().unwrap() else {
        panic!("expected Generic aggchain data");
    };

    let encoded = proto_generic
        .context
        .get("public_values")
        .expect("Generic v1 wire format always includes public_values");

    assert_eq!(encoded.as_ref(), expected_bytes.as_slice());
}

#[test]
fn multisig_and_aggchain_proof_v1_wire_uses_sp1_compatible_public_values_bytes() {
    let input = sample_multisig_and_aggchain_proof(Some(Box::new(sample_public_values())));

    let proto: v1::AggchainData = input.try_into().unwrap();
    let v1::aggchain_data::Data::MultisigAndAggchainProof(proto_with_multisig) =
        proto.data.as_ref().unwrap()
    else {
        panic!("expected MultisigAndAggchainProof aggchain data");
    };

    let encoded = proto_with_multisig
        .aggchain_proof
        .as_ref()
        .unwrap()
        .context
        .get("public_values")
        .expect("MultisigAndAggchainProof uses bare AggchainProof encoding when Some");

    assert_eq!(
        encoded.as_ref(),
        bare_public_values_expected_bytes().as_slice()
    );
}

#[test]
fn aggchain_proof_v1_decode_accepts_sp1_compatible_public_values_bytes() {
    use std::collections::HashMap;

    let public_values_bytes = bare_public_values_expected_bytes();

    let proto = v1::AggchainProof {
        proof: Some(sample_proof().try_into().unwrap()),
        aggchain_params: Some(Digest([0x88; 32]).into()),
        signature: None,
        context: HashMap::from([("public_values".to_owned(), public_values_bytes.into())]),
    };

    let decoded = AggchainProof::try_from(proto).unwrap();

    assert_eq!(
        decoded,
        sample_aggchain_proof(Some(Box::new(sample_public_values())))
    );
}

#[test]
fn generic_v1_decode_accepts_sp1_compatible_public_values_bytes() {
    use std::collections::HashMap;

    let bytes = generic_some_public_values_expected_bytes();

    let proto = v1::AggchainData {
        data: Some(v1::aggchain_data::Data::Generic(v1::AggchainProof {
            proof: Some(sample_proof().try_into().unwrap()),
            aggchain_params: Some(Digest([0x77; 32]).into()),
            signature: None,
            context: HashMap::from([("public_values".to_owned(), bytes.into())]),
        })),
    };

    let decoded = AggchainData::try_from(proto).unwrap();

    assert_eq!(
        decoded,
        sample_generic(Some(Box::new(sample_public_values())))
    );
}

#[rstest::rstest]
#[case("v4.0.0-rc.3", vec![0x04, 0x00, 0x03], vec![0xa4, 0x03])]
#[case("v6.0.0", vec![0x06, 0x00, 0x00, 0x01], vec![0xb6, 0x00, 0x02])]
fn sp1_stark_proof_round_trip_preserves_opaque_payload(
    #[case] version: &str,
    #[case] proof_bytes: Vec<u8>,
    #[case] vkey_bytes: Vec<u8>,
) {
    let proto = v1::Sp1StarkProof {
        version: version.to_owned(),
        proof: proof_bytes.clone().into(),
        vkey: vkey_bytes.clone().into(),
    };

    let proof = Proof::try_from(proto.clone()).unwrap();
    let v1::aggchain_proof::Proof::Sp1Stark(round_trip) =
        v1::aggchain_proof::Proof::try_from(proof).unwrap();

    assert_eq!(round_trip.version, proto.version);
    assert_eq!(round_trip.proof, proto.proof);
    assert_eq!(round_trip.vkey, proto.vkey);
}

#[test]
fn fuzz_round_trip_aggchain_data() {
    bolero::check!()
        .with_arbitrary::<AggchainData>()
        .for_each(|input| {
            let proto: v1::AggchainData = input.clone().try_into().unwrap();

            // Check if input has empty multisig signatures
            let has_empty_multisig = match &input {
                AggchainData::MultisigOnly { multisig } => multisig.0.is_empty(),
                AggchainData::MultisigAndAggchainProof { multisig, .. } => multisig.0.is_empty(),
                _ => false,
            };

            match AggchainData::try_from(proto) {
                Ok(output) => {
                    assert!(
                        !has_empty_multisig,
                        "Expected error for empty multisig signatures, but conversion succeeded"
                    );

                    assert_eq!(input, &output);
                }
                Err(err) => {
                    if has_empty_multisig {
                        let err_msg = err.to_string();
                        assert!(
                            err_msg.contains("Multisig ECDSA doesn't have any signature"),
                            "Expected empty multisig error, got: {err}",
                        );
                    } else {
                        panic!("Unexpected conversion error: {err}");
                    }
                }
            }
        })
}
make_round_trip_fuzzers!(fuzz_round_trip_bridge_exit, v1::BridgeExit, BridgeExit);
make_round_trip_fuzzers!(
    fuzz_round_trip_claim_from_mainnet,
    v1::ClaimFromMainnet,
    ClaimFromMainnet
);
make_round_trip_fuzzers!(
    fuzz_round_trip_claim_from_rollup,
    v1::ClaimFromRollup,
    ClaimFromRollup
);
make_round_trip_fuzzers!(fuzz_round_trip_digest, v1::FixedBytes32, Digest);
make_round_trip_fuzzers!(fuzz_round_trip_global_index, v1::FixedBytes32, GlobalIndex);
make_round_trip_fuzzers!(
    fuzz_round_trip_imported_bridge_exit,
    v1::ImportedBridgeExit,
    ImportedBridgeExit
);
make_round_trip_fuzzers!(
    fuzz_round_trip_l1_info_tree_leaf_with_context,
    v1::L1InfoTreeLeafWithContext,
    L1InfoTreeLeaf
);
make_round_trip_fuzzers!(
    fuzz_round_trip_l1_info_tree_leaf_inner,
    v1::L1InfoTreeLeaf,
    L1InfoTreeLeafInner
);
make_round_trip_fuzzers!(fuzz_round_trip_merkle_proof, v1::MerkleProof, MerkleProof);
make_round_trip_fuzzers!(fuzz_round_trip_token_info, v1::TokenInfo, TokenInfo);
make_round_trip_fuzzers!(fuzz_round_trip_u256, v1::FixedBytes32, U256);
