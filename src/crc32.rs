use super::{Algorithm, Crc, Digest};
use crate::table::crc32_table;
use crate::util::reflect_32;

impl Crc<u32> {
    pub const fn new(algorithm: &'static Algorithm<u32>) -> Self {
        let table = crc32_table(algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u32 {
        let mut crc = self.init();
        crc = self.update(crc, bytes);
        self.finalize(crc)
    }

    const fn init(&self) -> u32 {
        if self.algorithm.refin {
            reflect_32(self.algorithm.init)
        } else {
            self.algorithm.init
        }
    }

    const fn table_entry(&self, index: u32) -> u32 {
        self.table[(index & 0xFF) as usize]
    }

    const fn update(&self, mut crc: u32, bytes: &[u8]) -> u32 {
        let mut i = 0;
        if self.algorithm.refin {
            while i < bytes.len() {
                crc = self.table_entry(crc ^ bytes[i] as u32) ^ (crc >> 8);
                i += 1;
            }
        } else {
            while i < bytes.len() {
                crc = self.table_entry(bytes[i] as u32 ^ (crc >> 24)) ^ (crc << 8);
                i += 1;
            }
        }
        crc
    }

    const fn finalize(&self, mut crc: u32) -> u32 {
        if self.algorithm.refin ^ self.algorithm.refout {
            crc = reflect_32(crc);
        }
        crc ^ self.algorithm.xorout
    }

    pub const fn digest(&self) -> Digest<u32> {
        Digest::new(self)
    }
}

impl<'a> Digest<'a, u32> {
    const fn new(crc: &'a Crc<u32>) -> Self {
        let value = crc.init();
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u32 {
        self.crc.finalize(self.value)
    }
}
