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

## accelalate by x86intrin

We can use intel crc32 instruction by `RUSTFLAGS="-C target-feature=+sse4.2" cargo build --features=simd-accel`.

`cargo bench` with 2 GHz Intel Core i7 shows,

```
$ cargo bench
   Compiling crc v1.3.0 (file:///Users/hiroki.noda/dev/rust/crc-rs)
warning: field is never used: `poly`, #[warn(dead_code)] on by default
  --> src/crc32.rs:17:5
   |
17 |     poly: u32
   |     ^^^^^^^^^

warning: field is never used: `poly`, #[warn(dead_code)] on by default
  --> src/crc32.rs:17:5
   |
17 |     poly: u32
   |     ^^^^^^^^^

    Finished release [optimized] target(s) in 0.93 secs
     Running target/release/deps/bench-4ba45ff23227a84d

running 5 tests
test bench_crc32_castagnoli_update_megabytes ... bench:   2,918,740 ns/iter (+/- 825,319)
test bench_crc32_ieee_make_table             ... bench:       1,566 ns/iter (+/- 410)
test bench_crc32_ieee_update_megabytes       ... bench:   2,877,057 ns/iter (+/- 629,496)
test bench_crc64_make_table                  ... bench:       1,461 ns/iter (+/- 722)
test bench_crc64_update_megabytes            ... bench:   2,948,650 ns/iter (+/- 701,852)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured

$ RUSTFLAGS="-C target-feature=+sse4.2" cargo bench --features=simd-accel
   Compiling crc v1.3.0 (file:///Users/hiroki.noda/dev/rust/crc-rs)
    Finished release [optimized] target(s) in 1.1 secs
     Running target/release/deps/bench-ea8b558d9102b890

running 5 tests
test bench_crc32_castagnoli_update_megabytes ... bench:     431,005 ns/iter (+/- 35,214)
test bench_crc32_ieee_make_table             ... bench:       1,370 ns/iter (+/- 358)
test bench_crc32_ieee_update_megabytes       ... bench:   2,529,986 ns/iter (+/- 161,018)
test bench_crc64_make_table                  ... bench:       1,408 ns/iter (+/- 144)
test bench_crc64_update_megabytes            ... bench:   2,532,715 ns/iter (+/- 231,801)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured

     Running target/release/deps/crc-af46900232260aeb

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

## License

MIT
