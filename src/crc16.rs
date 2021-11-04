use super::{Algorithm, Crc, Digest};
use crate::table::crc16_table;

impl Crc<u16> {
    pub const fn new(algorithm: &'static Algorithm<u16>) -> Self {
        let table = crc16_table(algorithm.width, algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u16 {
        let mut crc = self.init();
        crc = self.update(crc, bytes);
        self.finalize(crc)
    }

    const fn init(&self) -> u16 {
        if self.algorithm.refin {
            self.algorithm.init.reverse_bits() >> (u16::BITS as u8 - self.algorithm.width)
        } else {
            self.algorithm.init << (u16::BITS as u8 - self.algorithm.width)
        }
    }

    const fn table_entry(&self, index: u16) -> u16 {
        self.table[(index & 0xFF) as usize]
    }

    const fn update(&self, mut crc: u16, bytes: &[u8]) -> u16 {
        let mut i = 0;
        if self.algorithm.refin {
            while i < bytes.len() {
                let table_index = crc ^ bytes[i] as u16;
                crc = self.table_entry(table_index) ^ (crc >> 8);
                i += 1;
            }
        } else {
            while i < bytes.len() {
                let table_index = (crc >> (u16::BITS - 8)) ^ bytes[i] as u16;
                crc = self.table_entry(table_index) ^ (crc << 8);
                i += 1;
            }
        }
        crc
    }

    const fn finalize(&self, mut crc: u16) -> u16 {
        if self.algorithm.refin ^ self.algorithm.refout {
            crc = crc.reverse_bits();
        }
        if !self.algorithm.refout {
            crc >>= u16::BITS as u8 - self.algorithm.width;
        }
        crc ^ self.algorithm.xorout
    }

    pub const fn digest(&self) -> Digest<u16> {
        Digest::new(self)
    }
}

impl<'a> Digest<'a, u16> {
    const fn new(crc: &'a Crc<u16>) -> Self {
        let value = crc.init();
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u16 {
        self.crc.finalize(self.value)
    }
}
