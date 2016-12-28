# crc [![Build Status](https://travis-ci.org/mrhooray/crc-rs.svg?branch=master)](https://travis-ci.org/mrhooray/crc-rs)
> Rust implementation of CRC(32, 64) with support of various standards

* [Crate](https://crates.io/crates/crc)
* [Documentation](http://mrhooray.github.io/crc-rs/crc/index.html)
* [Usage](#usage)
* [Benchmark](#benchmark)
* [License](#license)

## Usage

Add `crc` to `Cargo.toml`

```toml
[dependencies]
crc = "^1.0.0"
```

or

```toml
[dependencies.crc]
git = "https://github.com/mrhooray/crc-rs"
```

Add this to crate root

```rust
extern crate crc;
```

### Compute CRC32

```rust
use crc::{crc32, Hasher32};

// CRC-32-IEEE being the most commonly used one
assert_eq!(crc32::checksum_ieee(b"123456789"), 0xcbf43926);
assert_eq!(crc32::checksum_castagnoli(b"123456789"), 0xe3069283);
assert_eq!(crc32::checksum_koopman(b"123456789"), 0x2d3dd0ae);

// use provided or custom polynomial
let mut digest = crc32::Digest::new(crc32::IEEE);
digest.write(b"123456789");
assert_eq!(digest.sum32(), 0xcbf43926);

// with initial
let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, 0u32);
digest.write(b"123456789");
assert_eq!(digest.sum32(), 0xcbf43926);
```

### Compute CRC64

```rust
use crc::{crc64, Hasher64};

assert_eq!(crc64::checksum_ecma(b"123456789"), 0x995dc9bbdf1939fa);
assert_eq!(crc64::checksum_iso(b"123456789"), 0xb90956c775a41001);

// use provided or custom polynomial
let mut digest = crc64::Digest::new(crc64::ECMA);
digest.write(b"123456789");
assert_eq!(digest.sum64(), 0x995dc9bbdf1939fa);

// with initial
let mut digest = crc64::Digest::new_with_initial(crc64::ECMA, 0u64);
digest.write(b"123456789");
assert_eq!(digest.sum64(), 0x995dc9bbdf1939fa);
```

## accelalate by x86intrin

We can use intel crc32 instruction by `RUSTFLAGS="-C target-feature=+sse4.2" cargo build --features=simd-accel`.

## Benchmark

> Bencher is currently not available in Rust stable releases.

`cargo bench` with 2.3 GHz Intel Core i7 results ~430MB/s throughput. [Comparison](http://create.stephan-brumme.com/crc32/)

```
cargo bench
     Running target/release/bench-5c82e94dab3e9c79

running 4 tests
test bench_crc32_make_table       ... bench:       439 ns/iter (+/- 82)
test bench_crc32_update_megabytes ... bench:   2327803 ns/iter (+/- 138845)
test bench_crc64_make_table       ... bench:      1200 ns/iter (+/- 223)
test bench_crc64_update_megabytes ... bench:   2322472 ns/iter (+/- 92870)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured
```

## License

MIT
