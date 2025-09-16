use super::*;

use agglayer_primitives::U256;

#[test]
fn regression_sp1_serialization_roundtrip_fail() {
    // Conclusion: sp1 serialization is not deterministic, removed the equality
    // check.
    use bincode::Options;
    let bytes = hex::decode("00000000000000000000000000000000000000000000fb00000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000").unwrap();
    let input: SP1StarkProof = bincode::options()
        .deserialize(&bytes)
        .expect("failed first deserialization, would be fine");
    let serialized: Vec<u8> = crate::bincode::default()
        .serialize(&input)
        .expect("failed serialization, unexpected");
    let _output: SP1StarkProof = crate::bincode::default()
        .deserialize(&serialized)
        .expect("failed second deserialization, is unexpected");
}

#[rstest::rstest]
#[case(
    "multisig-empty",
    AggchainData::MultisigOnly { multisig: MultisigPayload(vec![]) },
)]
#[case(
    "multisig-none",
    AggchainData::MultisigOnly { multisig: MultisigPayload(vec![None]) },
)]
#[case(
    "multisig-onesig",
    AggchainData::MultisigOnly { multisig: MultisigPayload(vec![
        Some(Signature::new(U256::ZERO, U256::ZERO, false)),
    ]) },
)]
#[case(
    "ecdsa",
    AggchainData::ECDSA { signature: Signature::new(U256::ZERO, U256::ZERO, false),
})]
fn aggchaindata_json(#[case] name: &str, #[case] aggchain_data: AggchainData) {
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    struct Container {
        #[serde(flatten)]
        aggchain_data: AggchainData,
    }

    let container = Container { aggchain_data };
    let json_str = serde_json::to_string_pretty(&container).expect("serialization failed");
    insta::assert_snapshot!(name, json_str);
}
