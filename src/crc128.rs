use super::{Algorithm, Crc, Digest};
use crate::table::crc128_table;

impl Crc<u128> {
    pub const fn new(algorithm: &'static Algorithm<u128>) -> Self {
        let table = crc128_table(algorithm.width, algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u128 {
        let mut crc = self.init();
        crc = self.update(crc, bytes);
        self.finalize(crc)
    }

    const fn init(&self) -> u128 {
        if self.algorithm.refin {
            self.algorithm.init.reverse_bits() >> (128u8 - self.algorithm.width)
        } else {
            self.algorithm.init << (128u8 - self.algorithm.width)
        }
    }

    const fn table_entry(&self, index: u128) -> u128 {
        self.table[(index & 0xFF) as usize]
    }

    const fn update(&self, mut crc: u128, bytes: &[u8]) -> u128 {
        let mut i = 0;
        if self.algorithm.refin {
            while i < bytes.len() {
                let table_index = crc ^ bytes[i] as u128;
                crc = self.table_entry(table_index) ^ (crc >> 8);
                i += 1;
            }
        } else {
            while i < bytes.len() {
                let table_index = (crc >> 120) ^ bytes[i] as u128;
                crc = self.table_entry(table_index) ^ (crc << 8);
                i += 1;
            }
        }
        crc
    }

    const fn finalize(&self, mut crc: u128) -> u128 {
        if self.algorithm.refin ^ self.algorithm.refout {
            crc = crc.reverse_bits();
        }
        if !self.algorithm.refout {
            crc >>= 128u8 - self.algorithm.width;
        }
        crc ^ self.algorithm.xorout
    }

    pub const fn digest(&self) -> Digest<u128> {
        Digest::new(self)
    }
}

impl<'a> Digest<'a, u128> {
    const fn new(crc: &'a Crc<u128>) -> Self {
        let value = crc.init();
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn updated(mut self, bytes: &[u8]) -> Self {
        self.value = self.crc.update(self.value, bytes);
        self
    }

    pub const fn finalize(self) -> u128 {
        self.crc.finalize(self.value)
    }
}
