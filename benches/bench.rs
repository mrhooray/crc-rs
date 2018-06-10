#![feature(test)]
extern crate crc;
extern crate test;

use crc::{crc16, crc32, crc64};
use test::Bencher;

#[bench]
fn bench_crc16_make_table(b: &mut Bencher) {
    b.iter(|| crc16::make_table(crc16::X25, true));
}

#[bench]
fn bench_crc16_update_megabytes(b: &mut Bencher) {
    let table = crc16::make_table(crc16::X25, true);
    let bytes = Box::new([0u8; 1_000_000]);
    b.iter(|| crc16::update(0, &table, &*bytes, &crc::CalcType::Reverse));
}

#[bench]
fn bench_crc32_make_table(b: &mut Bencher) {
    b.iter(|| crc32::make_table(crc32::IEEE, true));
}

#[bench]
fn bench_crc32_update_megabytes(b: &mut Bencher) {
    let table = crc32::make_table(crc32::IEEE, true);
    let bytes = Box::new([0u8; 1_000_000]);
    b.iter(|| crc32::update(0, &table, &*bytes, &crc::CalcType::Reverse));
}

#[bench]
fn bench_crc64_make_table(b: &mut Bencher) {
    b.iter(|| crc64::make_table(crc64::ECMA, true));
}

#[bench]
fn bench_crc64_update_megabytes(b: &mut Bencher) {
    let table = crc64::make_table(crc64::ECMA, true);
    let bytes = Box::new([0u8; 1_000_000]);
    b.iter(|| crc64::update(0, &table, &*bytes, &crc::CalcType::Reverse));
}
