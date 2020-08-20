use super::{Algorithm, Crc, Digest};
use crate::table::crc16_table;
use crate::util::reflect_16;

impl Crc<u16> {
    pub const fn new(algorithm: &'static Algorithm<u16>) -> Self {
        let table = crc16_table(algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub fn checksum(&self, bytes: &[u8]) -> u16 {
        let mut crc = self.init();
        crc = self.update(crc, bytes);
        self.finalize(crc)
    }

    fn init(&self) -> u16 {
        if self.algorithm.refin {
            reflect_16(self.algorithm.init)
        } else {
            self.algorithm.init
        }
    }

    fn table_entry(&self, index: u16) -> u16 {
        self.table[(index & 0xFF) as usize]
    }

    fn update(&self, crc: u16, bytes: &[u8]) -> u16 {
        if self.algorithm.refin {
            bytes.iter().fold(crc, |crc, &byte| {
                self.table_entry(crc ^ byte as u16) ^ (crc >> 8)
            })
        } else {
            bytes.iter().fold(crc, |crc, &byte| {
                self.table_entry((byte as u16) ^ (crc >> 8)) ^ (crc << 8)
            })
        }
    }

    fn finalize(&self, mut crc: u16) -> u16 {
        if self.algorithm.refin ^ self.algorithm.refout {
            crc = reflect_16(crc);
        }
        crc ^ self.algorithm.xorout
    }

    pub fn digest(&self) -> Digest<u16> {
        Digest::new(self)
    }
}

impl<'a> Digest<'a, u16> {
    fn new(crc: &'a Crc<u16>) -> Self {
        let value = crc.init();
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub fn finalize(self) -> u16 {
        self.crc.finalize(self.value)
    }
}
