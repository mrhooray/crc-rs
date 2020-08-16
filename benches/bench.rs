use crc::*;
use criterion::{criterion_group, criterion_main};
use criterion::{Benchmark, Criterion, Throughput};

pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
pub const ECMA: Crc<u64> = Crc::<u64>::new(&CRC_64_ECMA_182);

fn crc16(c: &mut Criterion) {
    let mut digest = X25.digest();
    let bytes = vec![0u8; 1_000_000];
    c.bench(
        "crc16",
        Benchmark::new("crc16", move |b| b.iter(|| digest.update(&bytes)))
            .throughput(Throughput::Bytes(1_000_000)),
    );
}

fn crc32(c: &mut Criterion) {
    let mut digest = CASTAGNOLI.digest();
    let bytes = vec![0u8; 1_000_000];
    c.bench(
        "crc32",
        Benchmark::new("crc32", move |b| b.iter(|| digest.update(&bytes)))
            .throughput(Throughput::Bytes(1_000_000)),
    );
}

fn crc64(c: &mut Criterion) {
    let mut digest = ECMA.digest();
    let bytes = vec![0u8; 1_000_000];
    c.bench(
        "crc64",
        Benchmark::new("crc64", move |b| b.iter(|| digest.update(&bytes)))
            .throughput(Throughput::Bytes(1_000_000)),
    );
}

criterion_group!(crc16_benches, crc16);
criterion_group!(crc32_benches, crc32);
criterion_group!(crc64_benches, crc64);
criterion_main!(crc16_benches, crc32_benches, crc64_benches);
