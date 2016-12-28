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
    value: u32,
    poly: u32
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

#[cfg(feature = "simd-accel")]
#[cfg(target_feature = "sse4.2")]
#[inline(always)]
fn update_specialized_sse42(mut value: u32, bytes: &[u8]) -> u32 {
    use std::mem;
    use x86intrin::*;

    value = !value;
    if bytes.len() >= 16 {
        // Process unaligned bytes.
        let p = bytes as *const _;
        let mut i = unsafe { mem::transmute::<*const _, usize>(&p) } % 8;
        for e in 0 .. i {
            value = mm_crc32_u8(value, bytes[e]);
        }

        // Process 4 bytes at a time.
        while i + 8 <= bytes.len() {
            let v = [  bytes[i], bytes[i+1], bytes[i+2], bytes[i+3],
                     bytes[i+4], bytes[i+5], bytes[i+6], bytes[i+7]];
            value = mm_crc32_u64(value as u64, unsafe { mem::transmute::<[u8; 8], u64>(v) }) as u32;
            i += 8;
        }

        // Process 4 bytes at a time.
        while i + 4 <= bytes.len() {
            let v = [bytes[i], bytes[i+1], bytes[i+2], bytes[i+3]];
            value = mm_crc32_u32(value, unsafe { mem::transmute::<[u8; 4], u32>(v) });
            i += 4;
        }

        if bytes.len() - i > 0 {
            for &e in bytes[i .. ].into_iter() {
                value = mm_crc32_u8(value, e);
            }
        }
    } else {
        for &i in bytes.iter() {
            value = mm_crc32_u8(value, i);
        }
    }
    !value
}

#[cfg(feature = "simd-accel")]
#[cfg(target_feature = "sse4.2")]
pub fn checksum_castagnoli(bytes: &[u8]) -> u32 {
    return update_specialized_sse42(0, bytes);
}

#[cfg(not(feature = "simd-accel"))]
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
            value: 0,
            poly: poly
        }
    }

    pub fn new_with_initial(poly: u32, initial: u32) -> Digest {
        Digest {
            table: make_table(poly),
            initial: initial,
            value: initial,
            poly: poly
        }
    }
}

impl Hasher32 for Digest {
    fn reset(&mut self) {
        self.value = self.initial;
    }

    #[cfg(feature = "simd-accel")]
    #[cfg(target_feature = "sse4.2")]
    fn write(&mut self, bytes: &[u8]) {
        if self.poly == CASTAGNOLI {
            self.value = update_specialized_sse42(self.value, bytes);
        } else {
            self.value = update(self.value, &self.table, bytes);
        }
    }

    #[cfg(not(feature = "simd-accel"))]
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes);
    }

    fn sum32(&self) -> u32 {
        self.value
    }
}
