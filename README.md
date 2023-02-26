# crc

Rust implementation of CRC.

[![rust](https://github.com/mrhooray/crc-rs/actions/workflows/rust.yaml/badge.svg)](https://github.com/mrhooray/crc-rs/actions/workflows/rust.yaml)
[![Crate](https://img.shields.io/crates/v/crc.svg)](https://crates.io/crates/crc)
[![Docs](https://docs.rs/crc/badge.svg)](https://docs.rs/crc)
[![License](https://img.shields.io/crates/l/crc.svg?maxAge=2592000)](https://github.com/mrhooray/crc-rs#license)

### Usage

Add `crc` to `Cargo.toml`
```toml
[dependencies]
crc = "3.0"
```

### Examples

Using a well-known algorithm:
```rust
const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
assert_eq!(X25.checksum(b"123456789"), 0x906e);
```

Using a custom algorithm:
```rust
const CUSTOM_ALG: crc::Algorithm<u16> = crc::Algorithm {
    width: 16,
    poly: 0x8005,
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0xaee7,
    residue: 0x0000
};
let crc = crc::Crc::<u16>::new(&CUSTOM_ALG);
let mut digest = crc.digest();
digest.update(b"123456789");
assert_eq!(digest.finalize(), 0xaee7);
```

### Minimum supported Rust version (MSRV)

This crate's MSRV is 1.46.

At a minimum, the MSRV will be <= the oldest stable release in the last 12 months. MSRV may be bumped in minor version releases.

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

### Benchmark

`cargo bench` with AMD Ryzen 7 3800X ([comparison](http://create.stephan-brumme.com/crc32/)).

#### Throughput (GiB/s)

| Width | NoTable | Bytewise | Slice16 |
|-------|---------|----------|---------|
| 8     | 0.113   | 0.585    | 3.11    |
| 16    | 0.105   | 0.483    | 3.23    |
| 32    | 0.111   | 0.516    | 3.30    |
| 64    | 0.139   | 0.517    | 2.92    |
| 82    | 0.091   | 0.438    | 0.623   |

### License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
