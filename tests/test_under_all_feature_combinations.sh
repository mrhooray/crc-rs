set -ex

cargo test
cargo test --no-default-features

cargo test --features slice16-memory-restrictions
cargo test --features bytewise-memory-restrictions
cargo test --features no-table-memory-restrictions

cargo test --features bytewise-memory-restrictions,slice16-memory-restrictions
cargo test --features no-table-memory-restrictions,bytewise-memory-restrictions
cargo test --features no-table-memory-restrictions,slice16-memory-restrictions

cargo test --features no-table-memory-restrictions,bytewise-memory-restrictions,slice16-memory-restrictions

