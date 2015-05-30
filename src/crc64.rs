pub const ECMA: u64 = 0xc96c5795d7870f42;
pub const ISO: u64 = 0xd800000000000000;

lazy_static! {
    static ref ECMA_TABLE: [u64; 256] = make_table(ECMA);
    static ref ISO_TABLE: [u64; 256] = make_table(ISO);
}

pub struct Digest {
    table: [u64; 256],
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
            value: 0,
        }
    }
}

impl Hasher64 for Digest {
    fn reset(&mut self) {
        self.value = 0;
    }
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes);
    }
    fn sum64(&self) -> u64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ECMA_CHECK_VALUE: u64 = 0x995dc9bbdf1939fa;
    const ISO_CHECK_VALUE: u64 = 0xb90956c775a41001;

    #[test]
    fn test_checksum_ecma() {
        assert_eq!(checksum_ecma(b"123456789"), ECMA_CHECK_VALUE)
    }

    #[test]
    fn test_checksum_iso() {
        assert_eq!(checksum_iso(b"123456789"), ISO_CHECK_VALUE)
    }

    #[test]
    fn test_digest_ecma() {
        verify_checksum(ECMA, ECMA_CHECK_VALUE);
    }

    #[test]
    fn test_digest_iso() {
        verify_checksum(ISO, ISO_CHECK_VALUE);
    }

    fn verify_checksum(poly: u64, check_value: u64) {
        let mut digest = Digest::new(poly);
        digest.write(b"123456789");
        assert_eq!(digest.sum64(), check_value);
        digest.reset();
        for i in 1..10 {
            digest.write(i.to_string().as_bytes());
        }
        assert_eq!(digest.sum64(), check_value);
    }
}
