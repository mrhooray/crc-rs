# crc

[![rust](https://github.com/mrhooray/crc-rs/actions/workflows/rust.yaml/badge.svg)](https://github.com/mrhooray/crc-rs/actions/workflows/rust.yaml)
[![Crate](https://img.shields.io/crates/v/crc.svg)](https://crates.io/crates/crc)
[![Docs](https://docs.rs/crc/badge.svg)](https://docs.rs/crc)
[![License](https://img.shields.io/crates/l/crc.svg?maxAge=2592000)](https://github.com/mrhooray/crc-rs#license)

Rust implementation of CRC. MSRV is 1.46.

## Usage
Add `crc` to `Cargo.toml`
```toml
[dependencies]
crc = "3.0"
```

### Compute CRC

```rust
use crc::{Crc, Algorithm, CRC_16_IBM_SDLC, CRC_32_ISCSI};

pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

assert_eq!(X25.checksum(b"123456789"), 0x906e);
assert_eq!(CASTAGNOLI.checksum(b"123456789"), 0xe3069283);

// use custom algorithm
const CUSTOM_ALG: Algorithm<u16> = Algorithm {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xaee7,
    residue: 0x0000
};
let crc = Crc::<u16>::new(&CUSTOM_ALG);
let mut digest = crc.digest();
digest.update(b"123456789");
assert_eq!(digest.finalize(), 0xaee7);
``` 

### Lookup table flavors

This crate offers three flavors of lookup tables providing a tradeoff between computation speed and used memory.
See the benchmark section for hints, but do benchmarks on your target hardware to decide if the tradeoff is worth it to you.

1. `NoTable` provides an implementation that uses no additional memory
2. `Bytewise` provides an implementation that uses a lookup table that uses 256 entries of the used width (e.g. for u32 thats 256 * 4 bytes)
3. `Slice16` provides an implementation that uses a lookup table that uses 16 * 256 entries of the used width (e.g. for u32 thats 16 * 256 * 4 bytes)

These can be used by substituting `Crc<uxxx>` with e.g. `Crc<Slice16<uxxx>>`. If you use `Crc<uxxx>` the default implementation is used which are as follows:

* u8 -> Slice16
* u16 -> Slice16
* u32 -> Slice16
* u64 -> Slice16
* u128 -> Bytewise

Note that these tables can bloat your binary size if you precalculate them at compiletime (this happens in `Crc::new`). 
Choosing a crate like oncecell or lazystatic to compute them once at runtime may be preferable where binary size is a concern.

## Benchmark

`cargo bench` with AMD Ryzen 7 3800X. [Comparison](http://create.stephan-brumme.com/crc32/)

### With no lookup table

```
crc8/nolookup           time:   [138.00 µs 138.17 µs 138.35 µs]
                        thrpt:  [112.93 MiB/s 113.08 MiB/s 113.22 MiB/s]

crc16/nolookup          time:   [147.89 µs 148.00 µs 148.12 µs]
                        thrpt:  [105.49 MiB/s 105.57 MiB/s 105.65 MiB/s]


crc32/nolookup          time:   [140.10 µs 140.37 µs 140.62 µs]
                        thrpt:  [111.11 MiB/s 111.31 MiB/s 111.52 MiB/s]

crc64/nolookup          time:   [112.02 µs 112.06 µs 112.10 µs]
                        thrpt:  [139.39 MiB/s 139.44 MiB/s 139.49 MiB/s]

crc82/nolookup          time:   [171.19 µs 171.50 µs 171.84 µs]
                        thrpt:  [90.929 MiB/s 91.109 MiB/s 91.270 MiB/s]


```

### With 256 entry lookup table

```
crc8/bytewise           time:   [26.670 µs 26.699 µs 26.732 µs]
                        thrpt:  [584.51 MiB/s 585.22 MiB/s 585.87 MiB/s]

crc16/bytewise          time:   [32.303 µs 32.320 µs 32.338 µs]
                        thrpt:  [483.17 MiB/s 483.45 MiB/s 483.70 MiB/s]

crc32/bytewise          time:   [30.284 µs 30.309 µs 30.339 µs]
                        thrpt:  [515.02 MiB/s 515.52 MiB/s 515.95 MiB/s]

crc64/bytewise          time:   [30.218 µs 30.223 µs 30.227 µs]
                        thrpt:  [516.92 MiB/s 517.00 MiB/s 517.07 MiB/s]

crc82/bytewise          time:   [35.603 µs 35.670 µs 35.724 µs]
                        thrpt:  [437.39 MiB/s 438.05 MiB/s 438.87 MiB/s]
```

## With 16 x 256 entry lookup table

```
crc8/slice16            time:   [4.8891 µs 4.9057 µs 4.9250 µs]
                        thrpt:  [3.0982 GiB/s 3.1104 GiB/s 3.1210 GiB/s]

crc16/slice16           time:   [4.7201 µs 4.7235 µs 4.7277 µs]
                        thrpt:  [3.2276 GiB/s 3.2304 GiB/s 3.2327 GiB/s]

crc32/slice16           time:   [4.6134 µs 4.6217 µs 4.6302 µs]
                        thrpt:  [3.2955 GiB/s 3.3015 GiB/s 3.3075 GiB/s]

crc64/slice16           time:   [5.2283 µs 5.2303 µs 5.2324 µs]
                        thrpt:  [2.9162 GiB/s 2.9174 GiB/s 2.9185 GiB/s]

crc82/slice16           time:   [25.055 µs 25.061 µs 25.067 µs]
                        thrpt:  [623.32 MiB/s 623.47 MiB/s 623.64 MiB/s]
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
