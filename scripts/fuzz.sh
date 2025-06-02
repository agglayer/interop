#!/usr/bin/env bash
set -euixo pipefail

time="$1"
fuzzers=(
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_address"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_aggchain_data"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_bridge_exit"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_claim_from_mainnet"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_claim_from_rollup"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_digest"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_global_index"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_imported_bridge_exit"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_l1_info_tree_leaf_with_context"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_l1_info_tree_leaf_inner"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_merkle_proof"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_token_info"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_u256"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_parser_address"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_aggchain_data"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_bridge_exit"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_claim_from_mainnet"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_claim_from_rollup"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_digest"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_global_index"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_imported_bridge_exit"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_l1_info_tree_leaf_with_context"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_l1_info_tree_leaf_inner"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_merkle_proof"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_token_info"
    "agglayer-interop-grpc-types/compat::v1::tests::fuzz_round_trip_u256"
)

printf '%s\0' "${fuzzers[@]}" | parallel --null --bar --joblog fuzz.log bash -c '
    crate="$(echo {} | cut -d/ -f1)"
    target="$(echo {} | cut -d/ -f2)"
    cargo bolero test --rustc-bootstrap -p "$crate" --all-features "$target" -T '"$time"'
'
