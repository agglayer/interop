use std::{collections::BTreeMap, hash::Hash};

use agglayer_primitives::{keccak::keccak256_combine, Digest};
use rand::{
    prelude::{IndexedRandom as _, SliceRandom as _},
    random, rng, Rng,
};
use rs_merkle::{Hasher as MerkleHasher, MerkleTree};
use tiny_keccak::{Hasher as _, Keccak};

use crate::{error::SmtError, proof::ToBits, smt::Smt, utils::empty_hash_at_height};

const DEPTH: usize = 32;

#[derive(Clone, Debug)]
pub struct TestKeccak256;

impl MerkleHasher for TestKeccak256 {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        let mut keccak256 = Keccak::v256();
        keccak256.update(data);
        let mut output = [0u8; 32];
        keccak256.finalize(&mut output);
        output
    }
}

fn check_no_duplicates<A: Eq + Hash, B>(v: &[(A, B)]) {
    let mut seen = std::collections::HashSet::new();
    for (a, _) in v {
        assert!(seen.insert(a), "Duplicate key. Check your rng.");
    }
}

#[test]
fn test_compare_with_other_impl() {
    const DEPTH: usize = 8;
    let mut rng = rng();
    let num_keys = rng.random_range(0..=1 << DEPTH);
    let mut smt = Smt::<DEPTH>::new();
    let mut kvs: Vec<_> = (0..=u8::MAX).map(|i| (i, random())).collect();
    kvs.shuffle(&mut rng);
    for (key, value) in &kvs[..num_keys] {
        smt.insert(*key, *value).unwrap();
    }

    let mut leaves = vec![[0_u8; 32]; 1 << DEPTH];
    for (key, value) in &kvs[..num_keys] {
        leaves[key.reverse_bits() as usize] = **value;
    }
    let mt: MerkleTree<TestKeccak256> = MerkleTree::from_leaves(&leaves);

    assert_eq!(smt.root, mt.root().unwrap().into());
}

#[test]
fn test_order_consistency() {
    let mut rng = rng();
    let num_keys = rng.random_range(0..100);
    let mut smt = Smt::<DEPTH>::new();
    let mut kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&kvs);
    for (key, value) in kvs.iter() {
        smt.insert(*key, *value).unwrap();
    }
    let mut shuffled_smt = Smt::<DEPTH>::new();
    kvs.shuffle(&mut rng);
    for (key, value) in kvs.iter() {
        shuffled_smt.insert(*key, *value).unwrap();
    }

    assert_eq!(smt.root, shuffled_smt.root);
}

#[test]
fn test_inclusion_proof() {
    let mut rng = rng();
    let num_keys = rng.random_range(1..100);
    let mut smt = Smt::<DEPTH>::new();
    let kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&kvs);
    for (key, value) in kvs.iter() {
        smt.insert(*key, *value).unwrap();
    }
    let (key, value) = *kvs.choose(&mut rng).unwrap();
    let proof = smt.get_inclusion_proof(key).unwrap();
    assert!(proof.verify(key, value, smt.root));
}

#[test]
fn test_inclusion_proof_wrong_value() {
    let mut rng = rng();
    let num_keys = rng.random_range(1..100);
    let mut smt = Smt::<DEPTH>::new();
    let kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&kvs);
    for (key, value) in kvs.iter() {
        smt.insert(*key, *value).unwrap();
    }
    let (key, real_value) = *kvs.choose(&mut rng).unwrap();
    let proof = smt.get_inclusion_proof(key).unwrap();
    let fake_value = random();
    assert_ne!(real_value, fake_value, "Check your rng");
    assert!(!proof.verify(key, fake_value, smt.root));
}

#[test]
fn test_non_inclusion_proof() {
    let mut rng = rng();
    let num_keys = rng.random_range(0..100);
    let mut smt = Smt::<DEPTH>::new();
    let kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&kvs);
    for (key, value) in kvs.iter() {
        smt.insert(*key, *value).unwrap();
    }
    let key: u32 = random();
    assert!(!kvs.iter().any(|(k, _)| k == &key), "Check your rng");
    let proof = smt.get_non_inclusion_proof(key).unwrap();
    assert!(proof.verify(key, smt.root));
}

#[test]
fn test_non_inclusion_proof_failing() {
    let mut rng = rng();
    let num_keys = rng.random_range(1..100);
    let mut smt = Smt::<DEPTH>::new();
    let kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&kvs);
    for (key, value) in kvs.iter() {
        smt.insert(*key, *value).unwrap();
    }
    let (key, _) = *kvs.choose(&mut rng).unwrap();
    let error = smt.get_non_inclusion_proof(key).unwrap_err();
    assert_eq!(error, SmtError::KeyPresent);
}

#[test]
fn test_non_inclusion_proof_regression() {
    let mut smt = Smt::<DEPTH>::new();
    smt.insert(
        0,
        keccak256_combine([empty_hash_at_height::<1>(), empty_hash_at_height::<1>()]),
    )
    .unwrap();
    smt.insert(
        1 << (DEPTH - 1),
        keccak256_combine([empty_hash_at_height::<1>(), empty_hash_at_height::<1>()]),
    )
    .unwrap();
    let error = smt.get_non_inclusion_proof(0).unwrap_err();
    assert_eq!(error, SmtError::KeyPresent);
}

fn test_non_inclusion_proof_and_update(num_keys: usize) {
    let mut smt = Smt::<DEPTH>::new();
    let kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&kvs);
    for (key, value) in kvs.iter() {
        smt.insert(*key, *value).unwrap();
    }
    let key: u32 = random();
    assert!(!kvs.iter().any(|(k, _)| k == &key), "Check your rng");
    let proof = smt.get_non_inclusion_proof(key).unwrap();
    assert!(proof.verify(key, smt.root));
    let value = random();
    let new_root = proof.verify_and_update(key, value, smt.root).unwrap();
    smt.insert(key, value).unwrap();
    assert_eq!(smt.root, new_root);
}

#[test]
fn test_non_inclusion_proof_and_update_empty() {
    test_non_inclusion_proof_and_update(0)
}

#[test]
fn test_non_inclusion_proof_and_update_nonempty() {
    let num_keys = rng().random_range(1..100);
    test_non_inclusion_proof_and_update(num_keys)
}

#[test]
fn test_inclusion_proof_and_update() {
    let num_keys = rng().random_range(1..100);
    let mut smt = Smt::<DEPTH>::new();
    let kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&kvs);
    for (key, value) in kvs.iter() {
        smt.insert(*key, *value).unwrap();
    }
    let (key, value) = kvs[rng().random_range(0..num_keys)];
    let proof = smt.get_inclusion_proof(key).unwrap();
    assert!(proof.verify(key, value, smt.root));
    let new_value = random();
    let new_root = proof
        .verify_and_update(key, value, new_value, smt.root)
        .unwrap();
    smt.update(key, new_value).unwrap();
    assert_eq!(smt.root, new_root);
}

#[test]
fn test_inclusion_proof_zero_doesnt_update() {
    let mut smt = Smt::<DEPTH>::new();
    let num_keys = rng().random_range(1..100);
    let kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&kvs);
    for (key, value) in kvs.iter() {
        smt.insert(*key, *value).unwrap();
    }
    let (key, value) = kvs[rng().random_range(0..num_keys)];
    assert_ne!(
        value,
        Smt::<DEPTH>::EMPTY_HASH_ARRAY_AT_HEIGHT[0],
        "Check your rng"
    );
    let root = smt.root;
    let proof = smt.get_inclusion_proof_zero(key);
    assert!(proof.is_err(), "The key is in the SMT");
    assert_eq!(root, smt.root, "The SMT should not be updated");
}

#[test]
fn test_traverse_and_prune() {
    let mut rng = rng();
    let num_keys = rng.random_range(0..100);
    let kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&kvs);

    let mut smt0 = Smt::<DEPTH>::new();
    let mut smt1 = Smt::<DEPTH>::new();
    for (key, value) in kvs {
        smt0.insert(key, value).unwrap();
        smt1.insert(key, value).unwrap();
    }

    smt0.traverse_and_prune().unwrap();

    let other_kvs: Vec<(u32, _)> = (0..num_keys).map(|_| (random(), random())).collect();
    check_no_duplicates(&other_kvs);

    for (key, value) in other_kvs {
        smt0.insert(key, value).unwrap();
        smt1.insert(key, value).unwrap();
    }

    smt0.traverse_and_prune().unwrap();
    smt1.traverse_and_prune().unwrap();

    assert_eq!(smt0.root, smt1.root);
    assert_eq!(smt0.tree, smt1.tree);
}

#[test]
fn test_entries_simple() {
    let mut smt = Smt::<DEPTH>::new();

    let key = 42u32;
    let value = Digest([0xa; 32]);
    smt.insert(key, value).unwrap();

    let entries = smt.entries().unwrap();
    assert_eq!(entries.len(), 1);

    let (retrieved_path, retrieved_val) = entries[0];
    assert_eq!(retrieved_val, value);
    assert_eq!(retrieved_path, key.to_bits());
}

#[test]
fn test_entries_empty() {
    let smt = Smt::<DEPTH>::new();
    let entries = smt.entries().unwrap();
    assert!(entries.is_empty());
}

#[test]
fn test_entries_multiple() {
    let mut smt = Smt::<DEPTH>::new();

    let leaves = vec![
        (1u32, Digest([0xa; 32])),
        (2u32, Digest([0xb; 32])),
        (100u32, Digest([0xc; 32])),
    ];

    for (k, v) in &leaves {
        smt.insert(*k, *v).unwrap();
    }

    let got: BTreeMap<_, _> = smt.entries().unwrap().into_iter().collect();
    let expected: BTreeMap<_, _> = leaves.into_iter().map(|(k, v)| (k.to_bits(), v)).collect();

    assert_eq!(got, expected);
}

#[test]
fn test_entries_update_existing() {
    let mut smt = Smt::<DEPTH>::new();
    let key = 42u32;
    let val1 = Digest([0xa; 32]);
    let val2 = Digest([0xb; 32]);

    smt.insert(key, val1).unwrap();
    assert_eq!(1, smt.entries().unwrap().len());

    smt.update(key, val2).unwrap();
    let entries = smt.entries().unwrap();
    // update one balance shouldnt add new entry
    assert_eq!(1, entries.len());

    let (path, val) = entries[0];
    assert_eq!(path, key.to_bits());
    assert_eq!(val, val2);
}

#[test]
fn test_entries_non_zero_values() {
    let mut smt = Smt::<DEPTH>::new();
    let key = 42u32;
    let val = Digest([0xa; 32]);

    smt.insert(key, val).unwrap();
    assert_eq!(1, smt.entries().unwrap().len());

    smt.update(key, Digest::ZERO).unwrap();
    // nothing returned because the balance is zero now
    assert_eq!(0, smt.entries().unwrap().len());
}
