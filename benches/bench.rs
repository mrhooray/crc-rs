use crc::*;
use criterion::{criterion_group, criterion_main};
use criterion::{Criterion, Throughput};

pub const BLUETOOTH: Crc<u8> = Crc::<u8>::new(&CRC_8_BLUETOOTH);
pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
pub const GSM_40: Crc<u64> = Crc::<u64>::new(&CRC_40_GSM);
pub const ECMA: Crc<u64> = Crc::<u64>::new(&CRC_64_ECMA_182);
pub const DARC: Crc<u128> = Crc::<u128>::new(&CRC_82_DARC);

const INPUT_SIZE: usize = 1_000_000;

fn crc8(c: &mut Criterion) {
    let mut digest = BLUETOOTH.digest();
    let bytes = vec![0u8; INPUT_SIZE];
    let mut group = c.benchmark_group("crc8");
    group
        .throughput(Throughput::Bytes(bytes.len() as u64))
        .bench_function("crc8", move |b| b.iter(|| digest.update(&bytes)));
}

fn crc16(c: &mut Criterion) {
    let mut digest = X25.digest();
    let bytes = vec![0u8; INPUT_SIZE];
    let mut group = c.benchmark_group("crc16");
    group
        .throughput(Throughput::Bytes(bytes.len() as u64))
        .bench_function("crc16", move |b| b.iter(|| digest.update(&bytes)));
}

fn crc32(c: &mut Criterion) {
    let mut digest = CASTAGNOLI.digest();
    let bytes = vec![0u8; INPUT_SIZE];
    let mut group = c.benchmark_group("crc32");
    group
        .throughput(Throughput::Bytes(bytes.len() as u64))
        .bench_function("crc32", move |b| b.iter(|| digest.update(&bytes)));
}

fn crc40(c: &mut Criterion) {
    let mut digest = GSM_40.digest();
    let bytes = vec![0u8; INPUT_SIZE];
    let mut group = c.benchmark_group("crc40");
    group
        .throughput(Throughput::Bytes(bytes.len() as u64))
        .bench_function("crc40", move |b| b.iter(|| digest.update(&bytes)));
}

fn crc64(c: &mut Criterion) {
    let mut digest = ECMA.digest();
    let bytes = vec![0u8; INPUT_SIZE];
    let mut group = c.benchmark_group("crc64");
    group
        .throughput(Throughput::Bytes(bytes.len() as u64))
        .bench_function("crc64", move |b| b.iter(|| digest.update(&bytes)));
}

fn crc82(c: &mut Criterion) {
    let mut digest = ECMA.digest();
    let bytes = vec![0u8; INPUT_SIZE];
    let mut group = c.benchmark_group("crc82");
    group
        .throughput(Throughput::Bytes(bytes.len() as u64))
        .bench_function("crc82", move |b| b.iter(|| digest.update(&bytes)));
}

criterion_group!(crc8_benches, crc8);
criterion_group!(crc16_benches, crc16);
criterion_group!(crc32_benches, crc32);
criterion_group!(crc40_benches, crc40);
criterion_group!(crc64_benches, crc64);
criterion_group!(crc82_benches, crc82);
criterion_main!(
    crc8_benches,
    crc16_benches,
    crc32_benches,
    crc40_benches,
    crc64_benches,
    crc82_benches,
);
