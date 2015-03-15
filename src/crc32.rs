pub const CASTAGNOLI: u32 = 0x82f63b78;
pub const IEEE: u32 = 0xedb88320;
pub const KOOPMAN: u32 = 0xeb31d82e;

pub struct Digest {
    table: [u32; 256],
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

impl Digest {
    pub fn new(poly: u32) -> Digest {
        Digest {
            table: make_table(poly),
            value: 0,
        }
    }
}

impl Hasher32 for Digest {
    fn reset(&mut self) {
        self.value = 0;
    }
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes);
    }
    fn sum32(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_castagnoli() {
        verify_checksum(CASTAGNOLI, 0xe3069283);
    }

    #[test]
    fn test_ieee() {
        verify_checksum(IEEE, 0xcbf43926);
    }

    #[test]
    fn test_koopman() {
        verify_checksum(KOOPMAN, 0x2d3dd0ae);
    }

    #[bench]
    fn bench_make_table(b: &mut Bencher) {
        b.iter(|| make_table(IEEE));
    }

    #[bench]
    fn bench_digest_new(b: &mut Bencher) {
        b.iter(|| Digest::new(IEEE));
    }

    #[bench]
    fn bench_digest_write_megabytes(b: &mut Bencher) {
        let mut digest = Digest::new(IEEE);
        let bytes = Box::new([0u8; 1_000_000]);
        b.iter(|| digest.write(&*bytes));
    }

    fn verify_checksum(poly: u32, check_value: u32) {
        let mut digest = Digest::new(poly);
        digest.write(String::from_str("123456789").as_bytes());
        assert_eq!(digest.sum32(), check_value);
        digest.reset();
        for i in 1..10 {
            digest.write(i.to_string().as_bytes());
        }
        assert_eq!(digest.sum32(), check_value);
    }
}
