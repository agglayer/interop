use agglayer_interop_types::{
    aggchain_proof::{AggchainData, Proof},
    primitives::SignatureError,
    Address, BridgeExit, ClaimFromMainnet, ClaimFromRollup, Digest, GlobalIndex,
    ImportedBridgeExit, L1InfoTreeLeaf, L1InfoTreeLeafInner, MerkleProof, TokenInfo, U256,
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
