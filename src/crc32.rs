#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

pub use util::make_table_crc32 as make_table;
pub use util::CalcType;

include!(concat!(env!("OUT_DIR"), "/crc32_constants.rs"));

/// Structure that holds all of the important values for calculating a CRC
/// ### Details
/// - **table**: Holds the table values based on the supplied polynomial for the fast CRC calculations
/// - **initial**: The initial input value. AKA *reflect_in*
/// - **value**: Holds the current value of the CRC
/// - **reflect**: Chooses whether or not the CRC math is *Normal* or *Reverse*
/// - **final_xor**: Final value to XOR with when calling `Digest::sum32()`. AKA *reflect_out*
pub struct Digest {
    table: [u32; 256],
    initial: u32,
    value: u32,
    reflect: CalcType,
    final_xor: u32,
}

pub trait Hasher32 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum32(&self) -> u32;
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

/// Generates a generic IEEE 32 bit CRC (AKA CRC32).
pub fn checksum_ieee(bytes: &[u8]) -> u32 {
    return update(0u32, &IEEE_TABLE, bytes, &CalcType::Compat);
}

/// Generates a generic Castagnoli 32 bit CRC (AKA CRC32-C).
pub fn checksum_castagnoli(bytes: &[u8]) -> u32 {
    return update(0u32, &CASTAGNOLI_TABLE, bytes, &CalcType::Compat);
}

/// Generates a generic Koopman 32 bit CRC (AKA CRC32-K)
pub fn checksum_koopman(bytes: &[u8]) -> u32 {
    return update(0u32, &KOOPMAN_TABLE, bytes, &CalcType::Compat);
}

impl Digest {
    /// Creates a new table from the supplied polynomial and reflect parameter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc32, Hasher32};
    /// let mut digest = crc32::Digest::new(crc32::IEEE);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum32(), 0xcbf43926);;
    /// ```
    pub fn new(poly: u32) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: 0u32,
            value: 0u32,
            reflect: CalcType::Compat,
            final_xor: 0u32,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, and an initial value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc32, Hasher32};
    /// let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, 0u32);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum32(), 0xcbf43926);
    /// ```
    pub fn new_with_initial(poly: u32, initial: u32) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial,
            value: initial,
            reflect: CalcType::Compat,
            final_xor: 0u32,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, initial value, and final XOR value.
    /// ### Details
    /// This should be the default way to generate a custom CRC32.  See default values here: *http://crccalc.com/*.
    /// The example will generate a standard CRC32 table.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc32, Hasher32};
    /// let mut digest = crc32::Digest::new_custom(crc32::IEEE, 0xFFFFFFFF, crc32::CalcType::Reverse, 0xFFFFFFFF);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum32(), 0xcbf43926);
    /// ```
    pub fn new_custom(poly: u32, initial: u32, reflect: CalcType, final_xor: u32) -> Digest {
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

impl Hasher32 for Digest {
    /// Resets the current CRC in *value* to the *initial* value
    fn reset(&mut self) {
        self.value = self.initial;
    }

    /// Takes in a byte array and updates the CRC from based on the `Digest::reflect` field
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, &self.reflect);
    }

    /// Returns the current CRC after being XOR'd with the final XOR value
    fn sum32(&self) -> u32 {
        self.value ^ self.final_xor
    }
}

/// Implementation of `std::hash::Hasher` so that types which #[derive(Hash)] can hash with Digest.
impl Hasher for Digest {
    fn finish(&self) -> u64 {
        self.sum32() as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        Hasher32::write(self, bytes);
    }
}
