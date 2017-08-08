#![no_std]
extern crate crc;

use crc::{crc32, crc64};

pub fn foo() {
    crc32::make_table(crc32::IEEE);
    crc64::make_table(crc64::ECMA);
}
