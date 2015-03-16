#![feature(test)]
#[cfg(test)] extern crate test;

pub mod crc32;
pub mod crc64;

pub use self::crc32::Hasher32;
pub use self::crc64::Hasher64;
