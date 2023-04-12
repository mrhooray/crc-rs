set -ex

cargo test
cargo test --no-default-features

cargo test --features slice16-defaults
cargo test --features bytewise-defaults
cargo test --features notable-defaults

cargo test --features bytewise-defaults,slice16-defaults
cargo test --features notable-defaults,bytewise-defaults
cargo test --features notable-defaults,slice16-defaults

cargo test --features notable-defaults,bytewise-defaults,slice16-defaults

