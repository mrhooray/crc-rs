#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

use super::CalcType;
pub use util::make_table_crc64 as make_table;

include!(concat!(env!("OUT_DIR"), "/crc64_constants.rs"));

/// `Digest` struct for CRC calculation
/// - `table`: Calculation table generated from input parameters.
/// - `initial`: Initial value.
/// - `value`: Current value of the CRC calculation.
/// - `final_xor`: Final value to XOR with when calling `Digest::sum64()`.
/// - `calc`: Type of calculation. See its documentation for details.
pub struct Digest {
    table: [u64; 256],
    initial: u64,
    value: u64,
    final_xor: u64,
    calc: CalcType,
}

pub trait Hasher64 {
    /// Resets CRC calculation to `initial` value
    fn reset(&mut self);
    /// Updates CRC calculation with input byte array `bytes`
    fn write(&mut self, bytes: &[u8]);
    /// Returns checksum after being XOR'd with `final_xor`
    fn sum64(&self) -> u64;
}

/// Updates input CRC value `value` using CRC table `table` with byte array `bytes`.
pub fn update(mut value: u64, table: &[u64; 256], bytes: &[u8], calc: &CalcType) -> u64 {
    match calc {
        CalcType::Normal => {
            value = bytes.iter().fold(value, |acc, &x| {
                (acc << 8) ^ (table[((u64::from(x)) ^ (acc >> 56)) as usize])
            })
        }
        CalcType::Reverse => {
            value = bytes.iter().fold(value, |acc, &x| {
                (acc >> 8) ^ (table[((acc ^ (u64::from(x))) & 0xFF) as usize])
            })
        }
        CalcType::Compat => {
            value = !value;
            value = bytes.iter().fold(value, |acc, &x| {
                (acc >> 8) ^ (table[((acc ^ (u64::from(x))) & 0xFF) as usize])
            });
            value = !value;
        }
    }

    value
}

/// Generates a ECMA-188 64 bit CRC checksum (AKA CRC-64-ECMA).
pub fn checksum_ecma(bytes: &[u8]) -> u64 {
    update(0u64, &ECMA_TABLE, bytes, &CalcType::Compat)
}

/// Generates a ISO 3309 32 bit CRC checksum (AKA CRC-64-ISO).
pub fn checksum_iso(bytes: &[u8]) -> u64 {
    update(0u64, &ISO_TABLE, bytes, &CalcType::Compat)
}

impl Digest {
    /// Creates a new Digest from input polynomial.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc64, Hasher64};
    /// let mut digest = crc64::Digest::new(crc64::ECMA);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum64(), 0x995dc9bbdf1939fa);;
    /// ```
    pub fn new(poly: u64) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: 0u64,
            value: 0u64,
            final_xor: 0u64,
            calc: CalcType::Compat,
        }
    }

    /// Creates a new Digest from input polynomial and initial value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc64, Hasher64};
    /// let mut digest = crc64::Digest::new_with_initial(crc64::ECMA, 0u64);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum64(), 0x995dc9bbdf1939fa);
    /// ```
    pub fn new_with_initial(poly: u64, initial: u64) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial,
            value: initial,
            final_xor: 0u64,
            calc: CalcType::Compat,
        }
    }

    /// Creates a fully customized Digest from input parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc64, Hasher64};
    /// let mut digest = crc64::Digest::new_custom(crc64::ECMA, !0u64, !0u64, crc::CalcType::Reverse);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum64(), 0x995dc9bbdf1939fa);
    /// ```
    pub fn new_custom(poly: u64, initial: u64, final_xor: u64, calc: CalcType) -> Digest {
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

impl Hasher64 for Digest {
    fn reset(&mut self) {
        self.value = self.initial;
    }

    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, &self.calc);
    }

    fn sum64(&self) -> u64 {
        self.value ^ self.final_xor
    }
}

impl Hasher for Digest {
    fn finish(&self) -> u64 {
        self.sum64()
    }

    fn write(&mut self, bytes: &[u8]) {
        Hasher64::write(self, bytes);
    }
}
