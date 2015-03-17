pub const CASTAGNOLI: u32 = 0x82f63b78;
pub const IEEE: u32 = 0xedb88320;
pub const KOOPMAN: u32 = 0xeb31d82e;

lazy_static! {
    static ref IEEE_TABLE: [u32; 256] = make_table(IEEE);
    static ref CASTAGNOLI_TABLE: [u32; 256] = make_table(CASTAGNOLI);
    static ref KOOPMAN_TABLE: [u32; 256] = make_table(KOOPMAN);
}

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

    const CASTAGNOLI_CHECK_VALUE: u32 = 0xe3069283;
    const IEEE_CHECK_VALUE: u32 = 0xcbf43926;
    const KOOPMAN_CHECK_VALUE: u32 = 0x2d3dd0ae;

    #[test]
    fn test_checksum_castagnoli() {
        assert_eq!(checksum_castagnoli(&b"123456789"), CASTAGNOLI_CHECK_VALUE)
    }

    #[test]
    fn test_checksum_ieee() {
        assert_eq!(checksum_ieee(&b"123456789"), IEEE_CHECK_VALUE)
    }

    #[test]
    fn test_checksum_koopman() {
        assert_eq!(checksum_koopman(&b"123456789"), KOOPMAN_CHECK_VALUE)
    }

    #[test]
    fn test_digest_castagnoli() {
        verify_checksum(CASTAGNOLI, CASTAGNOLI_CHECK_VALUE);
    }

    #[test]
    fn test_digest_ieee() {
        verify_checksum(IEEE, IEEE_CHECK_VALUE);
    }

    #[test]
    fn test_digest_koopman() {
        verify_checksum(KOOPMAN, KOOPMAN_CHECK_VALUE);
    }


    #[bench]
    fn bench_make_table(b: &mut Bencher) {
        b.iter(|| make_table(IEEE));
    }

    #[bench]
    fn bench_update_megabytes(b: &mut Bencher) {
        let table = make_table(IEEE);
        let bytes = Box::new([0u8; 1_000_000]);
        b.iter(|| update(0, &table, &*bytes));
    }

    fn verify_checksum(poly: u32, check_value: u32) {
        let mut digest = Digest::new(poly);
        digest.write(&b"123456789");
        assert_eq!(digest.sum32(), check_value);
        digest.reset();
        for i in 1..10 {
            digest.write(i.to_string().as_bytes());
        }
        assert_eq!(digest.sum32(), check_value);
    }
}
