use crc::{crc16, crc32, crc64};
use criterion::{criterion_group, criterion_main};
use criterion::{Benchmark, Criterion, Throughput};

fn crc16_make_table(c: &mut Criterion) {
    c.bench_function("crc16_make_table", |b| {
        b.iter(|| crc16::make_table(crc16::X25, true))
    });
}

fn crc16_update_megabytes(c: &mut Criterion) {
    let table = crc16::make_table(crc16::X25, true);
    let bytes = Box::new([0u8; 1_000_000]);
    c.bench(
        "crc16_update_megabytes",
        Benchmark::new("crc16_update_megabytes", move |b| {
            b.iter(|| crc16::update(0, &table, &*bytes, &crc::CalcType::Reverse))
        })
        .throughput(Throughput::Bytes(1_000_000)),
    );
}

fn crc32_make_table(c: &mut Criterion) {
    c.bench_function("crc32_make_table", |b| {
        b.iter(|| crc32::make_table(crc32::IEEE, true))
    });
}

fn crc32_update_megabytes(c: &mut Criterion) {
    let table = crc32::make_table(crc32::IEEE, true);
    let bytes = Box::new([0u8; 1_000_000]);
    c.bench(
        "crc32_update_megabytes",
        Benchmark::new("crc32_update_megabytes", move |b| {
            b.iter(|| crc32::update(0, &table, &*bytes, &crc::CalcType::Reverse))
        })
        .throughput(Throughput::Bytes(1_000_000)),
    );
}

fn crc64_make_table(c: &mut Criterion) {
    c.bench_function("crc64_make_table", |b| {
        b.iter(|| crc64::make_table(crc64::ECMA, true))
    });
}

fn crc64_update_megabytes(c: &mut Criterion) {
    let table = crc64::make_table(crc64::ECMA, true);
    let bytes = Box::new([0u8; 1_000_000]);
    c.bench(
        "crc64_update_megabytes",
        Benchmark::new("crc64_update_megabytes", move |b| {
            b.iter(|| crc64::update(0, &table, &*bytes, &crc::CalcType::Reverse))
        })
        .throughput(Throughput::Bytes(1_000_000)),
    );
}

criterion_group!(crc16, crc16_make_table, crc16_update_megabytes);
criterion_group!(crc32, crc32_make_table, crc32_update_megabytes);
criterion_group!(crc64, crc64_make_table, crc64_update_megabytes);
criterion_main!(crc16, crc32, crc64);
