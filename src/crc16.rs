#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

use super::CalcType;
pub use util::make_table_crc16 as make_table;

include!(concat!(env!("OUT_DIR"), "/crc16_constants.rs"));

/// `Digest` struct for CRC calculation
/// - `table`: Calculation table generated from input parameters.
/// - `initial`: Initial value.
/// - `value`: Current value of the CRC calculation.
/// - `final_xor`: Final value to XOR with when calling `Digest::sum16()`.
/// - `calc`: Type of calculation. See its documentation for details.
pub struct Digest {
    table: [u16; 256],
    initial: u16,
    value: u16,
    final_xor: u16,
    calc: CalcType,
}

pub trait Hasher16 {
    /// Resets CRC calculation to `initial` value
    fn reset(&mut self);
    /// Updates CRC calculation with input byte array `bytes`
    fn write(&mut self, bytes: &[u8]);
    /// Returns checksum after being XOR'd with `final_xor`
    fn sum16(&self) -> u16;
}

/// Updates input CRC value `value` using CRC table `table` with byte array `bytes`.
pub fn update(mut value: u16, table: &[u16; 256], bytes: &[u8], calc: &CalcType) -> u16 {
    match calc {
        CalcType::Normal => {
            value = bytes.iter().fold(value, |acc, &x| {
                (acc << 8) ^ (table[((u16::from(x)) ^ (acc >> 8)) as usize])
            })
        }
        CalcType::Reverse => {
            value = bytes.iter().fold(value, |acc, &x| {
                (acc >> 8) ^ (table[((acc ^ (u16::from(x))) & 0xFF) as usize])
            })
        }
        CalcType::Compat => {
            value = !value;
            value = bytes.iter().fold(value, |acc, &x| {
                (acc >> 8) ^ (table[((acc ^ (u16::from(x))) & 0xFF) as usize])
            });
            value = !value;
        }
    }

    value
}

/// Generates a X25 16 bit CRC checksum (AKA CRC-16-CCITT).
pub fn checksum_x25(bytes: &[u8]) -> u16 {
    update(0u16, &X25_TABLE, bytes, &CalcType::Compat)
}

/// Generates a USB 16 bit CRC checksum (AKA CRC-16-IBM).
pub fn checksum_usb(bytes: &[u8]) -> u16 {
    update(0u16, &USB_TABLE, bytes, &CalcType::Compat)
}

impl Digest {
    /// Creates a new Digest from input polynomial.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc16, Hasher16};
    /// let mut digest = crc16::Digest::new(crc16::X25);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum16(), 0x906e);;
    /// ```
    pub fn new(poly: u16) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: 0u16,
            value: 0u16,
            final_xor: 0u16,
            calc: CalcType::Compat,
        }
    }

    /// Creates a new Digest from input polynomial and initial value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc16, Hasher16};
    /// let mut digest = crc16::Digest::new_with_initial(crc16::X25, 0u16);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum16(), 0x906e);
    /// ```
    pub fn new_with_initial(poly: u16, initial: u16) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial,
            value: initial,
            final_xor: 0u16,
            calc: CalcType::Compat,
        }
    }

    /// Creates a fully customized Digest from input parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc16, Hasher16};
    /// let mut digest = crc16::Digest::new_custom(crc16::X25, !0u16, !0u16, crc::CalcType::Reverse);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum16(), 0x906e);
    /// ```
    pub fn new_custom(poly: u16, initial: u16, final_xor: u16, calc: CalcType) -> Digest {
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

impl Hasher16 for Digest {
    fn reset(&mut self) {
        self.value = self.initial;
    }

    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, &self.calc);
    }

    fn sum16(&self) -> u16 {
        self.value ^ self.final_xor
    }
}

impl Hasher for Digest {
    fn finish(&self) -> u64 {
        u64::from(self.sum16())
    }

    fn write(&mut self, bytes: &[u8]) {
        Hasher16::write(self, bytes);
    }
}
