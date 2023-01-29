use crc::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

pub const BLUETOOTH: Crc<u8> = Crc::<u8>::new(&CRC_8_BLUETOOTH);
pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
pub const ISCSI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
pub const ISCSI_SLICE16: Crc<Slice16<u32>> = Crc::<Slice16<u32>>::new(&CRC_32_ISCSI);
pub const ISCSI_BYTEWISE: Crc<Bytewise<u32>> = Crc::<Bytewise<u32>>::new(&CRC_32_ISCSI);
pub const ISCSI_NOLOOKUP: Crc<NoTable<u32>> = Crc::<NoTable<u32>>::new(&CRC_32_ISCSI);
pub const GSM_40: Crc<u64> = Crc::<u64>::new(&CRC_40_GSM);
pub const ECMA: Crc<u64> = Crc::<u64>::new(&CRC_64_ECMA_182);
pub const DARC: Crc<u128> = Crc::<u128>::new(&CRC_82_DARC);

static KB: usize = 1024;

fn baseline(data: &[u8]) -> usize {
    data.iter()
        .fold(0usize, |acc, v| acc.wrapping_add(*v as usize))
}

fn checksum(c: &mut Criterion) {
    let size = 16 * KB;
    let bytes = vec![0u8; size];

    c.benchmark_group("baseline")
        .throughput(Throughput::Bytes(size as u64))
        .bench_function("baseline", |b| b.iter(|| baseline(black_box(&bytes))));

    c.benchmark_group("crc32")
        .throughput(Throughput::Bytes(size as u64))
        .bench_function("default", |b| b.iter(|| ISCSI.checksum(black_box(&bytes))))
        .bench_function("nolookup", |b| {
            b.iter(|| ISCSI_NOLOOKUP.checksum(black_box(&bytes)))
        })
        .bench_function("bytewise", |b| {
            b.iter(|| ISCSI_BYTEWISE.checksum(black_box(&bytes)))
        })
        .bench_function("slice16", |b| {
            b.iter(|| ISCSI_SLICE16.checksum(black_box(&bytes)))
        });

    c.benchmark_group("checksum")
        .bench_function("crc8", |b| b.iter(|| BLUETOOTH.checksum(black_box(&bytes))))
        .bench_function("crc16", |b| b.iter(|| X25.checksum(black_box(&bytes))))
        .bench_function("crc40", |b| b.iter(|| GSM_40.checksum(black_box(&bytes))))
        .bench_function("crc64", |b| b.iter(|| ECMA.checksum(black_box(&bytes))))
        .bench_function("crc82", |b| b.iter(|| DARC.checksum(black_box(&bytes))));
}

criterion_group!(checksum_benches, checksum);
criterion_main!(checksum_benches);
