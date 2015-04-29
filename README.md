# crc [![Build Status](https://travis-ci.org/mrhooray/crc-rs.svg?branch=master)](https://travis-ci.org/mrhooray/crc-rs)
> Rust implementation of CRC(32, 64) with support of various standards

* [Crate](https://crates.io/crates/crc)
* [Documentation](http://mrhooray.github.io/crc-rs/crc/index.html)
* [Usage](#usage)
* [Benchmark](#benchmark)
* [TODO](#todo)
* [License](#license)

##Usage
Add `crc` to `Cargo.toml`
```toml
[dependencies]
crc = "^0.3.1"
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
```

##Benchmark
`cargo bench` with 2.3 GHz Intel Core i7 results ~430MB/s throughput. [Comparison](http://create.stephan-brumme.com/crc32/)
```
cargo bench
     Running target/release/crc-ef3fd4a2422fe338

running 14 tests
test crc32::tests::test_checksum_castagnoli ... ignored
test crc32::tests::test_checksum_ieee ... ignored
test crc32::tests::test_checksum_koopman ... ignored
test crc32::tests::test_digest_castagnoli ... ignored
test crc32::tests::test_digest_ieee ... ignored
test crc32::tests::test_digest_koopman ... ignored
test crc64::tests::test_checksum_ecma ... ignored
test crc64::tests::test_checksum_iso ... ignored
test crc64::tests::test_digest_ecma ... ignored
test crc64::tests::test_digest_iso ... ignored
test crc32::tests::bench_make_table       ... bench:      2511 ns/iter (+/- 1256)
test crc32::tests::bench_update_megabytes ... bench:   2307480 ns/iter (+/- 483053)
test crc64::tests::bench_make_table       ... bench:      2510 ns/iter (+/- 928)
test crc64::tests::bench_update_megabytes ... bench:   2298012 ns/iter (+/- 78780)

test result: ok. 0 passed; 0 failed; 10 ignored; 4 measured
```

##TODO
- [ ] [Slicing-by-4/8/16](http://create.stephan-brumme.com/crc32/#slicing-by-8-overview)

##License
MIT
