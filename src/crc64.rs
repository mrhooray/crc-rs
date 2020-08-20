use super::{Algorithm, Crc, Digest};
use crate::table::crc64_table;
use crate::util::reflect_64;

impl Crc<u64> {
    pub const fn new(algorithm: &'static Algorithm<u64>) -> Self {
        let table = crc64_table(algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub fn checksum(&self, bytes: &[u8]) -> u64 {
        let mut crc = self.init();
        crc = self.update(crc, bytes);
        self.finalize(crc)
    }

    fn init(&self) -> u64 {
        if self.algorithm.refin {
            reflect_64(self.algorithm.init)
        } else {
            self.algorithm.init
        }
    }

    fn table_entry(&self, index: u64) -> u64 {
        self.table[(index & 0xFF) as usize]
    }

    fn update(&self, crc: u64, bytes: &[u8]) -> u64 {
        if self.algorithm.refin {
            bytes.iter().fold(crc, |crc, &byte| {
                self.table_entry(crc ^ byte as u64) ^ (crc >> 8)
            })
        } else {
            bytes.iter().fold(crc, |crc, &byte| {
                self.table_entry((byte as u64) ^ (crc >> 56)) ^ (crc << 8)
            })
        }
    }

    fn finalize(&self, mut crc: u64) -> u64 {
        if self.algorithm.refin ^ self.algorithm.refout {
            crc = reflect_64(crc);
        }
        crc ^ self.algorithm.xorout
    }

    pub fn digest(&self) -> Digest<u64> {
        Digest::new(self)
    }
}

impl<'a> Digest<'a, u64> {
    fn new(crc: &'a Crc<u64>) -> Self {
        let value = crc.init();
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub fn finalize(self) -> u64 {
        self.crc.finalize(self.value)
    }
}
