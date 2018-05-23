#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

pub use util::make_table_crc16 as make_table;

include!(concat!(env!("OUT_DIR"), "/crc16_constants.rs"));

pub struct Digest {
    table: [u16; 256],
    initial: u16,
    value: u16,
    reflect: bool,
    final_xor: u16,
}

pub trait Hasher16 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum16(&self) -> u16;
}

pub fn update(mut value: u16, table: &[u16; 256], bytes: &[u8], rfl: bool) -> u16 {
    let shift = 8;

    for &i in bytes.iter() {
        if true == rfl {
            value = table[((value ^ (i as u16)) & 0xFF) as usize] ^ (value >> 8)
        } else {
            value = table[(((value >> shift) as u8) ^ i) as usize] ^ (value << 8);
        }
    }
    value
}

pub fn checksum_x25(bytes: &[u8]) -> u16 {
    return update(0xFFFF, &X25_TABLE, bytes, true) ^ 0xFFFF;
}

pub fn checksum_usb(bytes: &[u8]) -> u16 {
    return update(0xFFFF, &USB_TABLE, bytes, true) ^ 0xFFFF;
}

impl Digest {
    pub fn new(poly: u16) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: 0xFFFF,
            value: 0xFFFF,
            reflect: true,
            final_xor: 0xFFFF,
        }
    }

    pub fn new_with_initial(poly: u16, initial: u16) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: initial,
            value: initial,
            reflect: true,
            final_xor: 0xFFFF,
        }
    }

    pub fn new_with_initial_and_final(
        poly: u16,
        initial: u16,
        reflect: bool,
        final_xor: u16,
    ) -> Digest {
        Digest {
            table: make_table(poly, reflect),
            initial: initial,
            value: initial,
            reflect: reflect,
            final_xor: final_xor,
        }
    }
}

impl Hasher16 for Digest {
    fn reset(&mut self) {
        self.value = self.initial;
    }
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, self.reflect);
    }
    fn sum16(&self) -> u16 {
        self.value ^ self.final_xor
    }
}

/// Implementation of std::hash::Hasher so that types which #[derive(Hash)] can hash with Digest.
impl Hasher for Digest {
    fn write(&mut self, bytes: &[u8]) {
        Hasher16::write(self, bytes);
    }

    fn finish(&self) -> u64 {
        self.sum16() as u64
    }
}
