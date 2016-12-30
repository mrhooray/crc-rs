use std::hash::Hasher;

pub const CASTAGNOLI: u32 = 0x82f63b78;
pub const IEEE: u32 = 0xedb88320;
pub const KOOPMAN: u32 = 0xeb31d82e;

lazy_static! {
    pub static ref IEEE_TABLE: [u32; 256] = make_table(IEEE);
    pub static ref CASTAGNOLI_TABLE: [u32; 256] = make_table(CASTAGNOLI);
    pub static ref KOOPMAN_TABLE: [u32; 256] = make_table(KOOPMAN);
}

pub struct Digest {
    table: [u32; 256],
    initial: u32,
    value: u32
}

pub trait Hasher32 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum32(&self) -> u32;
}

pub fn make_table(poly: u32) -> [u32; 256] {
    let mut table = [0u32; 256];
    for i in 0..256 {
        let mut value = i as u32;
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

pub fn update(mut value: u32, table: &[u32; 256], bytes: &[u8]) -> u32 {
    value = !value;
    for &i in bytes.iter() {
        value = table[((value as u8) ^ i) as usize] ^ (value >> 8)
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
