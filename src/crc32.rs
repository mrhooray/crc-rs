use core::hash::Hasher;

pub use crate::util::make_table_crc32 as make_table;
use crate::CalcType;

pub const CASTAGNOLI: u32 = 0x1EDC6F41;
pub const CASTAGNOLI_TABLE: [u32; 256] = make_table(CASTAGNOLI, true);
pub const IEEE: u32 = 0x04C11DB7;
pub const IEEE_TABLE: [u32; 256] = make_table(IEEE, true);
pub const KOOPMAN: u32 = 0x741B8CD7;
pub const KOOPMAN_TABLE: [u32; 256] = make_table(KOOPMAN, true);

/// `Digest` struct for CRC calculation
/// - `table`: Calculation table generated from input parameters.
/// - `initial`: Initial value.
/// - `value`: Current value of the CRC calculation.
/// - `final_xor`: Final value to XOR with when calling `Digest::sum32()`.
/// - `calc`: Type of calculation. See its documentation for details.
pub struct Digest {
    table: [u32; 256],
    initial: u32,
    value: u32,
    final_xor: u32,
    calc: CalcType,
}

pub trait Hasher32 {
    /// Resets CRC calculation to `initial` value
    fn reset(&mut self);
    /// Updates CRC calculation with input byte array `bytes`
    fn write(&mut self, bytes: &[u8]);
    /// Returns checksum after being XOR'd with `final_xor`
    fn sum32(&self) -> u32;
}

/// Updates input CRC value `value` using CRC table `table` with byte array `bytes`.
pub fn update(mut value: u32, table: &[u32; 256], bytes: &[u8], calc: &CalcType) -> u32 {
    match calc {
        CalcType::Normal => {
            value = bytes.iter().fold(value, |acc, &x| {
                (acc << 8) ^ (table[((u32::from(x)) ^ (acc >> 24)) as usize])
            })
        }
        CalcType::Reverse => {
            value = bytes.iter().fold(value, |acc, &x| {
                (acc >> 8) ^ (table[((acc ^ (u32::from(x))) & 0xFF) as usize])
            })
        }
        CalcType::Compat => {
            value = !value;
            value = bytes.iter().fold(value, |acc, &x| {
                (acc >> 8) ^ (table[((acc ^ (u32::from(x))) & 0xFF) as usize])
            });
            value = !value;
        }
    }

    value
}

/// Generates a IEEE 32 bit CRC checksum (AKA CRC32).
pub fn checksum_ieee(bytes: &[u8]) -> u32 {
    update(0u32, &IEEE_TABLE, bytes, &CalcType::Compat)
}

/// Generates a Castagnoli 32 bit CRC checksum (AKA CRC32-C).
pub fn checksum_castagnoli(bytes: &[u8]) -> u32 {
    update(0u32, &CASTAGNOLI_TABLE, bytes, &CalcType::Compat)
}

/// Generates a Koopman 32 bit CRC checksum (AKA CRC32-K).
pub fn checksum_koopman(bytes: &[u8]) -> u32 {
    update(0u32, &KOOPMAN_TABLE, bytes, &CalcType::Compat)
}

impl Digest {
    /// Creates a new Digest from input polynomial.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc32, Hasher32};
    /// let mut digest = crc32::Digest::new(crc32::IEEE);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum32(), 0xcbf43926);;
    /// ```
    pub const fn new(poly: u32) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: 0u32,
            value: 0u32,
            final_xor: 0u32,
            calc: CalcType::Compat,
        }
    }

    /// Creates a new Digest from input polynomial and initial value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc32, Hasher32};
    /// let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, 0u32);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum32(), 0xcbf43926);
    /// ```
    pub const fn new_with_initial(poly: u32, initial: u32) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial,
            value: initial,
            final_xor: 0u32,
            calc: CalcType::Compat,
        }
    }

    /// Creates a fully customized Digest from input parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc32, Hasher32};
    /// let mut digest = crc32::Digest::new_custom(crc32::IEEE, !0u32, !0u32, crc::CalcType::Reverse);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum32(), 0xcbf43926);
    /// ```
    pub fn new_custom(poly: u32, initial: u32, final_xor: u32, calc: CalcType) -> Digest {
        let mut rfl: bool = true;
        if let CalcType::Normal = calc {
            rfl = false;
        }

        Digest {
            table: make_table(poly, rfl),
            initial,
            value: initial,
            final_xor,
            calc,
        }
    }
}

impl Hasher32 for Digest {
    fn reset(&mut self) {
        self.value = self.initial;
    }

    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, &self.calc);
    }

    fn sum32(&self) -> u32 {
        self.value ^ self.final_xor
    }
}

impl Hasher for Digest {
    fn finish(&self) -> u64 {
        u64::from(self.sum32())
    }

    fn write(&mut self, bytes: &[u8]) {
        Hasher32::write(self, bytes);
    }
}
