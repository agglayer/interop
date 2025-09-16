use agglayer_interop_types::{
    aggchain_proof::AggchainData, primitives::SignatureError, Address, BridgeExit,
    ClaimFromMainnet, ClaimFromRollup, Digest, GlobalIndex, ImportedBridgeExit, L1InfoTreeLeaf,
    L1InfoTreeLeafInner, MerkleProof, TokenInfo, U256,
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
                    // If we had empty multisig, this should not have succeeded
                    assert!(
                        !has_empty_multisig,
                        "Expected error for empty multisig signatures, but conversion succeeded"
                    );

                    // For most cases, we can't check equality due to stark proofs,
                    // but we can at least verify the conversion succeeded
                    if let (
                        AggchainData::ECDSA { signature: sig1 },
                        AggchainData::ECDSA { signature: sig2 },
                    ) = (&input, &output)
                    {
                        assert_eq!(sig1, sig2);
                    }
                }
                Err(err) => {
                    if has_empty_multisig {
                        // This error is expected for empty multisig cases
                        let err_msg = err.to_string();
                        assert!(
                            err_msg.contains("Multisig ECDSA doesn't have any signature"),
                            "Expected empty multisig error, got: {}",
                            err
                        );
                    } else {
                        // Any other error should cause the test to fail
                        panic!("Unexpected conversion error: {}", err);
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
