#![feature(test)]
extern crate crc;
extern crate test;

use crc::{crc32, crc64};
use test::Bencher;

#[bench]
fn bench_crc32_make_table(b: &mut Bencher) {
    b.iter(|| crc32::make_table(crc32::IEEE, true));
}

#[bench]
fn bench_crc32_update_megabytes(b: &mut Bencher) {
    let table = crc32::make_table(crc32::IEEE, true);
    let bytes = Box::new([0u8; 1_000_000]);
    b.iter(|| crc32::update(0xFFFFFFFF, &table, &*bytes, true) ^ 0xFFFFFFFF);
}

#[bench]
fn bench_crc64_make_table(b: &mut Bencher) {
    b.iter(|| crc64::make_table(crc64::ECMA, true));
}

#[bench]
fn bench_crc64_update_megabytes(b: &mut Bencher) {
    let table = crc64::make_table(crc64::ECMA, true);
    let bytes = Box::new([0u8; 1_000_000]);
    b.iter(|| crc64::update(0, &table, &*bytes, true));
}

/*
#[macro_use]
extern crate criterion;
extern crate crc;

use crc::{crc32, crc64};
use criterion::{Criterion, Fun};

/// Makes 4 benchmarks and runs them
fn benchmarks(c: &mut Criterion) {
    let bench_crc32 = Fun::new("CRC 32", |b, _i| {
        b.iter(|| crc32::make_table(crc32::IEEE, true))
    });

    let table = crc32::make_table(crc32::IEEE, true);
    let bytes = Box::new([0u8; 991_000]); // 1MB overflows the stack
    let bench_crc32_mb = Fun::new("CRC 32 991kB", move |b, _i| {
        b.iter(|| crc32::update(0, &table, &*bytes, true))
    });

    let bench_crc64 = Fun::new("CRC 64", |b, _i| {
        b.iter(|| crc64::make_table(crc64::ECMA, true))
    });

    let table2 = crc64::make_table(crc64::ECMA, true);
    let bytes2 = Box::new([0u8; 991_000]); // 1MB overflows the stack
    let bench_crc64_mb = Fun::new("CRC 64 991kB", move |b, _i| {
        b.iter(|| crc64::update(0, &table2, &*bytes2, true))
    });

    // Build up a vector of the benchmark functions
    let functions = vec![bench_crc32, bench_crc32_mb, bench_crc64, bench_crc64_mb];

    //Runs the benchmarks
    c.bench_functions("Benchmarks", functions, &20);
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
*/
