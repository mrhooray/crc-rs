use std::hash::Hasher;

pub const ECMA: u64 = 0xc96c5795d7870f42;
pub const ISO: u64 = 0xd800000000000000;

lazy_static! {
    pub static ref ECMA_TABLE: [u64; 256] = make_table(ECMA);
    pub static ref ISO_TABLE: [u64; 256] = make_table(ISO);
}

pub struct Digest {
    table: [u64; 256],
    initial: u64,
    value: u64
}

pub trait Hasher64 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum64(&self) -> u64;
}

pub fn make_table(poly: u64) -> [u64; 256] {
    let mut table = [0u64; 256];
    for i in 0..256 {
        let mut value = i as u64;
        for _ in 0..8 {
            value = if (value & 1) == 1 {
                (value >> 1) ^ poly
            } else {
                value >> 1
            }
        }
        table[i] = value;
    }
    table
}

pub fn update(mut value: u64, table: &[u64; 256], bytes: &[u8]) -> u64 {
    value = !value;
    for &i in bytes.iter() {
        value = table[((value as u8) ^ i) as usize] ^ (value >> 8)
    }
    !value
}

pub fn checksum_ecma(bytes: &[u8]) -> u64 {
    return update(0, &ECMA_TABLE, bytes);
}

pub fn checksum_iso(bytes: &[u8]) -> u64 {
    return update(0, &ISO_TABLE, bytes);
}

impl Digest {
    pub fn new(poly: u64) -> Digest {
        Digest {
            table: make_table(poly),
            initial: 0,
            value: 0
        }
    }

    pub fn new_with_initial(poly: u64, initial: u64) -> Digest {
        Digest {
            table: make_table(poly),
            initial: initial,
            value: initial
        }
    }
}

impl Hasher64 for Digest {
    fn reset(&mut self) {
        self.value = self.initial;
    }
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes);
    }
    fn sum64(&self) -> u64 {
        self.value
    }
}

/// Implementation of std::hash::Hasher so that types which #[derive(Hash)] can hash with Digest.
impl Hasher for Digest {
    fn write(&mut self, bytes: &[u8]) {
        Hasher64::write(self, bytes);
    }

    fn finish(&self) -> u64 {
        self.sum64()
    }
}
