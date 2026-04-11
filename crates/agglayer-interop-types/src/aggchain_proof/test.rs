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

#[test]
fn proof_bincode_round_trip_preserves_raw_envelope() {
    let proof = Proof::SP1Stark(SP1StarkWithContext {
        version: "v6.0.0".to_owned(),
        proof: vec![0xde, 0xad, 0xbe, 0xef],
        vkey: vec![0xca, 0xfe, 0xba, 0xbe],
    });

    let encoded = crate::bincode::default().serialize(&proof).unwrap();
    let decoded: Proof = crate::bincode::default().deserialize(&encoded).unwrap();

    match decoded {
        Proof::SP1Stark(decoded) => {
            assert_eq!(decoded.version, "v6.0.0");
            assert_eq!(decoded.proof, vec![0xde, 0xad, 0xbe, 0xef]);
            assert_eq!(decoded.vkey, vec![0xca, 0xfe, 0xba, 0xbe]);
        }
    }
}

#[test]
fn proof_serde_round_trip_preserves_raw_envelope() {
    let proof = Proof::SP1Stark(SP1StarkWithContext {
        version: "v4.0.0-rc.3".to_owned(),
        proof: vec![0xde, 0xad, 0xbe, 0xef],
        vkey: vec![0xca, 0xfe, 0xba, 0xbe],
    });

    let encoded = serde_json::to_string(&proof).unwrap();
    let decoded: Proof = serde_json::from_str(&encoded).unwrap();

    match decoded {
        Proof::SP1Stark(decoded) => {
            assert_eq!(decoded.version, "v4.0.0-rc.3");
            assert_eq!(decoded.proof, vec![0xde, 0xad, 0xbe, 0xef]);
            assert_eq!(decoded.vkey, vec![0xca, 0xfe, 0xba, 0xbe]);
        }
    }
}
