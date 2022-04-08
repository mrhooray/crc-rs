use super::{Algorithm, Crc, Digest};
use crate::table::crc8_table;

impl Crc<u8> {
    pub const fn new(algorithm: &'static Algorithm<u8>) -> Self {
        let table = crc8_table(algorithm.width, algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u8 {
        let mut crc = self.init();
        crc = self.update(crc, bytes);
        self.finalize(crc)
    }

    const fn init(&self) -> u8 {
        if self.algorithm.refin {
            self.algorithm.init.reverse_bits() >> (8u8 - self.algorithm.width)
        } else {
            self.algorithm.init << (8u8 - self.algorithm.width)
        }
    }

    const fn table_entry(&self, index: u8) -> u8 {
        self.table[index as usize]
    }

    const fn update(&self, mut crc: u8, bytes: &[u8]) -> u8 {
        let mut i = 0;

        while i < bytes.len() {
            crc = self.table_entry(crc ^ bytes[i]);
            i += 1;
        }

        crc
    }

    const fn finalize(&self, mut crc: u8) -> u8 {
        if self.algorithm.refin ^ self.algorithm.refout {
            crc = crc.reverse_bits();
        }
        if !self.algorithm.refout {
            crc >>= 8u8 - self.algorithm.width;
        }
        crc ^ self.algorithm.xorout
    }

    pub const fn digest(&self) -> Digest<u8> {
        let initial = self.init();
        Digest::new(self, initial)
    }

    pub const fn digest_with_initial(&self, initial: u8) -> Digest<u8> {
        Digest::new(self, initial)
    }
}

impl<'a> Digest<'a, u8> {
    const fn new(crc: &'a Crc<u8>, value: u8) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u8 {
        self.crc.finalize(self.value)
    }
}
