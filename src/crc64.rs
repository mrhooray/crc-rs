#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

pub use util::make_table_crc64 as make_table;

include!(concat!(env!("OUT_DIR"), "/crc64_constants.rs"));

pub struct Digest {
    table: [u64; 256],
    initial: u64,
    value: u64,
    reflect: bool,
    final_xor: u64,
}

pub trait Hasher64 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum64(&self) -> u64;
}

pub fn update(mut value: u64, table: &[u64; 256], bytes: &[u8], rfl: bool) -> u64 {
    let shift = 56;

    for &i in bytes.iter() {
        if true == rfl {
            value = table[((value ^ (i as u64)) & 0xFF) as usize] ^ (value >> 8)
        } else {
            value = table[(((value >> shift) as u8) ^ i) as usize] ^ (value << 8);
        }
    }

    value
}

pub fn checksum_ecma(bytes: &[u8]) -> u64 {
    return update(0xFFFFFFFFFFFFFFFF, &ECMA_TABLE, bytes, true) ^ 0xFFFFFFFFFFFFFFFF;
}

pub fn checksum_iso(bytes: &[u8]) -> u64 {
    return update(0xFFFFFFFFFFFFFFFF, &ISO_TABLE, bytes, true) ^ 0xFFFFFFFFFFFFFFFF;
}

impl Digest {
    pub fn new(poly: u64) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: 0xFFFFFFFFFFFFFFFF,
            value: 0xFFFFFFFFFFFFFFFF,
            reflect: true,
            final_xor: 0xFFFFFFFFFFFFFFFF,
        }
    }

    pub fn new_with_initial(poly: u64, initial: u64) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: initial,
            value: initial,
            reflect: true,
            final_xor: 0,

        }
    }

        pub fn new_with_initial_and_final(
        poly: u64,
        initial: u64,
        reflect: bool,
        final_xor: u64,
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

impl Hasher64 for Digest {
    fn reset(&mut self) {
        self.value = self.initial;
    }
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, self.reflect);
    }
    fn sum64(&self) -> u64 {
        self.value ^ self.final_xor
    }
}

/// Implementation of std::hash::Hasher so that types which #[derive(Hash)] can hash with Digest.
impl Hasher for Digest {
    fn write(&mut self, bytes: &[u8]) {
        Hasher64::write(self, bytes);
    }

    fn finish(&self) -> u64 {
        self.sum64() as u64
    }
}
