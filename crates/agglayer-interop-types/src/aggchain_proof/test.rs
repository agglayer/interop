use agglayer_primitives::U256;

use super::*;

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
