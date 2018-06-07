#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

pub use util::make_table_crc64 as make_table;
pub use util::CalcType;

include!(concat!(env!("OUT_DIR"), "/crc64_constants.rs"));

/// Structure that holds all of the important values for calculating a CRC
/// ### Details
/// - **table**: Holds the table values based on the supplied polynomial for the fast CRC calculations
/// - **initial**: The initial input value. AKA *reflect_in*
/// - **value**: Holds the current value of the CRC
/// - **reflect**: Chooses whether or not the CRC math is *Normal* or *Reverse*
/// - **final_xor**: Final value to XOR with when calling `Digest::sum64()`. AKA *reflect_out*
pub struct Digest {
    table: [u64; 256],
    initial: u64,
    value: u64,
    reflect: CalcType,
    final_xor: u64,
}

pub trait Hasher64 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum64(&self) -> u64;
}

/// Calculate the CRC of the byte string of values.
/// ### Details
/// Updates the current CRC *value* using the CRC table *table* using the byte array *bytes*.
/// The parameter *calc* will reflect the data.  *calc=Normal* will calculate the CRC MSB first.
/// *calc=Reverse* will calculate the CRC LSB first.  *calc=Compat* will calculate the CRC LSB first
/// and reflect *value* both in and out.
///
/// # Usage
///
/// call using Digest::write(&bytes)
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

/// Generates a generic ECMA-188 64 bit CRC (AKA CRC-64-ECMA).
pub fn checksum_ecma(bytes: &[u8]) -> u64 {
    return update(0u64, &ECMA_TABLE, bytes, &CalcType::Compat);
}

/// Generates a generic ISO 3309 32 bit CRC (AKA CRC-64-ISO).
pub fn checksum_iso(bytes: &[u8]) -> u64 {
    return update(0u64, &ISO_TABLE, bytes, &CalcType::Compat);
}

impl Digest {
    /// Creates a new table from the supplied polynomial and reflect parameter
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
            reflect: CalcType::Compat,
            final_xor: 0u64,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, and an initial value
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
            reflect: CalcType::Compat,
            final_xor: 0u64,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, initial value, and final XOR value
    /// ### Details
    /// This should be the default way to generate a custom CRC64.  See default values here: *http://crccalc.com/*
    /// The example will generate a standard CRC64 table.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc64, Hasher64};
    /// let mut digest = crc64::Digest::new_custom(crc64::ECMA, 0xFFFFFFFFFFFFFFFF, crc64::CalcType::Reverse, 0xFFFFFFFFFFFFFFFF);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum64(), 0x995dc9bbdf1939fa);
    /// ```
    pub fn new_custom(poly: u64, initial: u64, reflect: CalcType, final_xor: u64) -> Digest {
        let mut rfl: bool = true;
        if let CalcType::Normal = reflect {
            rfl = false;
        }

        Digest {
            table: make_table(poly, rfl),
            initial,
            value: initial,
            reflect,
            final_xor,
        }
    }
}

impl Hasher64 for Digest {
    /// Resets the current CRC in *value* to the *initial* value
    fn reset(&mut self) {
        self.value = self.initial;
    }

    /// Takes in a byte array and updates the CRC from based on the `Digest::reflect` field
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, &self.reflect);
    }

    /// Returns the current CRC after being XOR'd with the final XOR value
    fn sum64(&self) -> u64 {
        self.value ^ self.final_xor
    }
}

/// Implementation of `std::hash::Hasher` so that types which #[derive(Hash)] can hash with Digest.
impl Hasher for Digest {
    fn finish(&self) -> u64 {
        self.sum64() as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        Hasher64::write(self, bytes);
    }
}
