# Changelog

All notable changes to this project will be documented in this file.

## [0.16.0] - 2026-04-23

### 🐛 Bug Fixes

- Reject overflowing multisig signer indexes in gRPC compat decode (#217)
- *(grpc)* [**breaking**] Align aggchain public_values compat with SP1 codec (#218)

## [0.15.0] - 2026-04-14

### 🚀 Features

- Preserve opaque SP1 proof payloads in interop (#202)

## [0.14.0] - 2026-03-24

### 💼 Other

- *(deps)* Refresh Cargo.lock dependency versions (#187)

## [0.13.0] - 2025-12-25

### 🚀 Features

- Expose the SMT leaf values (#155)

### 💼 Other

- *(deps)* Bump the external group across 1 directory with 6 updates (#152)

## [0.11.0] - 2025-09-16

### 🐛 Bug Fixes

- Json serialization of `AggchainData` (#111)

## [0.10.0] - 2025-08-28

### 🚀 Features

- [**breaking**] Move all dynamic error types to eyre (#88)
- Adding proto multisig definition (#83)

## [0.9.0] - 2025-07-23

### 🚜 Refactor

- Wrap `alloy_primitives::Address` and give data types specific `serde` names (#40)

## [0.7.1] - 2025-06-04

### 🚀 Features

- Expose the aggchain proof public values (#35)

## [0.6.1] - 2025-05-28

### 🚀 Features

- Adding changelogs and configure agglayer-primitives' (#32)
- Publish 0.6.0 (#33)
- Add typed tree roots (#31)

## [0.6.0] - 2025-05-20

### 🚀 Features

- Use strong types for rollup indexes (#21)

### 🐛 Bug Fixes

- Make signature field in AggchainData optional (#29)

## [0.4.0] - 2025-05-16

### 🚀 Features

- Add signature field to AggchainProof message (#24)

### 🐛 Bug Fixes

- Do run all tests in ci (#22)

## [0.2.0] - 2025-04-02

### 🚀 Features

- Introduce fuzzer infra, make everything faster (#9)
- Updating aggchain-proof format (#11)

### 🚜 Refactor

- Remove pessimistic-proof and split crates


