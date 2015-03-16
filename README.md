# crc
> Rust implementation of CRC(32, 64)

* [Crate](https://crates.io/crates/crc)
* [Usage](#usage)
* [Benchmark](#benchmark)
* [TODO](#todo)
* [License](#license)

##Usage
Add `crc` to `Cargo.toml`
```toml
[dependencies]
crc = "0.0.1"
```
or
```toml
[dependencies.crc]
git = "https://github.com/mrhooray/crc"
```

Add this to crate root
```rust
extern crate crc;
```

Compute CRC32
```rust
use crc::crc32;

let mut digest = crc32::Digest::new(crc32::IEEE);
digest.write(&b"123456789");
println!("{:x}", digest.sum32()); // -> 0xcbf43926
```

Compute CRC64
```rust
use crc::crc64;

let mut digest = crc64::Digest::new(crc64::ECMA);
digest.write(&b"123456789");
println!("{:x}", digest.sum64()); // -> 0x995dc9bbdf1939fa
```

##Benchmark
`cargo bench` with 2.3 GHz Intel Core i7 results ~385MB/s throughput as [expected](http://create.stephan-brumme.com/crc32/)
```
running 11 tests
test crc32::tests::test_castagnoli ... ignored
test crc32::tests::test_ieee ... ignored
test crc32::tests::test_koopman ... ignored
test crc64::tests::test_ecma ... ignored
test crc64::tests::test_iso ... ignored
test crc32::tests::bench_digest_new             ... bench:       455 ns/iter (+/- 193)
test crc32::tests::bench_digest_write_megabytes ... bench:   2590734 ns/iter (+/- 354853)
test crc32::tests::bench_make_table             ... bench:       443 ns/iter (+/- 62)
test crc64::tests::bench_digest_new             ... bench:      1259 ns/iter (+/- 162)
test crc64::tests::bench_digest_write_megabytes ... bench:   2602540 ns/iter (+/- 292056)
test crc64::tests::bench_make_table             ... bench:      1275 ns/iter (+/- 181)

test result: ok. 0 passed; 0 failed; 5 ignored; 6 measured
```

##TODO
- [ ] [Slicing-by-4/8/16](http://create.stephan-brumme.com/crc32/#slicing-by-8-overview) Implementation

##License
MIT
