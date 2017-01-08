use std::hash::Hasher;

use byteorder::{ByteOrder, NativeEndian};

pub const CASTAGNOLI: u32 = 0x82f63b78;
pub const IEEE: u32 = 0xedb88320;
pub const KOOPMAN: u32 = 0xeb31d82e;

lazy_static! {
    pub static ref IEEE_TABLE: [[u32; 256]; 8] = make_table(IEEE);
    pub static ref CASTAGNOLI_TABLE: [[u32; 256]; 8] = make_table(CASTAGNOLI);
    pub static ref KOOPMAN_TABLE: [[u32; 256]; 8] = make_table(KOOPMAN);
}

pub struct Digest {
    table: [[u32; 256]; 8],
    initial: u32,
    value: u32
}

pub trait Hasher32 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum32(&self) -> u32;
}

pub fn make_table(poly: u32) -> [[u32; 256]; 8] {
    let mut table = [[0u32; 256]; 8];
    for i in 0..256 {
        let mut value = i as u32;
        for _ in 0..8 {
            value = if (value & 1) == 1 {
                (value >> 1) ^ poly
            } else {
                value >> 1
            }
        }
        table[0][i] = value;
    }

    for i in 0..256 {
        table[1][i] = (table[0][i] >> 8) ^ table[0][(table[0][i] & 0xFF) as usize];
        table[2][i] = (table[1][i] >> 8) ^ table[0][(table[1][i] & 0xFF) as usize];
        table[3][i] = (table[2][i] >> 8) ^ table[0][(table[2][i] & 0xFF) as usize];
        table[4][i] = (table[3][i] >> 8) ^ table[0][(table[3][i] & 0xFF) as usize];
        table[5][i] = (table[4][i] >> 8) ^ table[0][(table[4][i] & 0xFF) as usize];
        table[6][i] = (table[5][i] >> 8) ^ table[0][(table[5][i] & 0xFF) as usize];
        table[7][i] = (table[6][i] >> 8) ^ table[0][(table[6][i] & 0xFF) as usize];
    }
    table
}

pub fn update(mut value: u32, table: &[[u32; 256]; 8], bytes: &[u8]) -> u32 {
    value = !value;
    let mut i = 0;
    while bytes.len() - i >= 8 {
        let one: u32 = NativeEndian::read_u32(&bytes[i..i+4]) ^ value;
        let two: u32 = NativeEndian::read_u32(&bytes[i+4..i+8]);
        value =
            table[0][(two >> 24) as usize] ^
            table[1][((two >> 16) & 0xFF) as usize] ^
            table[2][((two >> 8) & 0xFF) as usize] ^
            table[3][(two & 0xFF) as usize] ^
            table[4][(one >> 24) as usize] ^
            table[5][((one >> 16) & 0xFF) as usize] ^
            table[6][((one >> 8) & 0xFF) as usize] ^
            table[7][(one & 0xFF) as usize];
            i += 8;
    }
    for &e in bytes[i..].into_iter() {
        value = table[0][((value as u8) ^ e) as usize] ^ (value >> 8)
    }
    !value
}

pub fn checksum_ieee(bytes: &[u8]) -> u32 {
    return update(0, &IEEE_TABLE, bytes);
}

pub fn checksum_castagnoli(bytes: &[u8]) -> u32 {
    return update(0, &CASTAGNOLI_TABLE, bytes);
}

pub fn checksum_koopman(bytes: &[u8]) -> u32 {
    return update(0, &KOOPMAN_TABLE, bytes);
}

impl Digest {
    pub fn new(poly: u32) -> Digest {
        Digest {
            table: make_table(poly),
            initial: 0,
            value: 0
        }
    }

    pub fn new_with_initial(poly: u32, initial: u32) -> Digest {
        Digest {
            table: make_table(poly),
            initial: initial,
            value: initial
        }
    }
}

impl Hasher32 for Digest {
    fn reset(&mut self) {
        self.value = self.initial;
    }
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes);
    }
    fn sum32(&self) -> u32 {
        self.value
    }
}

/// Implementation of std::hash::Hasher so that types which #[derive(Hash)] can hash with Digest.
impl Hasher for Digest {
    fn write(&mut self, bytes: &[u8]) {
        Hasher32::write(self, bytes);
    }

    fn finish(&self) -> u64 {
        self.sum32() as u64
    }
}
