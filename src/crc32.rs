#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

pub use util::make_table_crc32 as make_table;

include!(concat!(env!("OUT_DIR"), "/crc32_constants.rs"));

/// Structure that holds all of the important values for calculating a CRC
///
/// # Definitions
///
/// *table:* Holds the table values based on the supplied polynomial for the fast CRC calculations
///
/// *initial:* The initial inut value. AKA reflect_in
///
/// *value:* Holds the current value of the CRC
///
/// *reflect:* Chooses whether or not the CRC math is normal or reflected
///
/// *final_xor:* Final value to XOR with when calling Digest::sum32
pub struct Digest {
    table: [u32; 256],
    initial: u32,
    value: u32,
    reflect: bool,
    final_xor: u32,
}

pub trait Hasher32 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum32(&self) -> u32;
}

/// Caclulate the CRC of the byte string of values.
///
/// Updates the current CRC *value* using the CRC table *table* using the byte array *bytes*.
/// The parameter *rfl* will reflect the data.  *rfl=false* will calculate the CRC MSB first.
/// *rfl=true* will calculate the CRC LSB first.
///
/// # Usage
///
/// call using Digest::write(&bytes)
pub fn update(mut value: u32, table: &[u32; 256], bytes: &[u8], rfl: bool) -> u32 {
    let shift = 24;

    for &i in bytes.iter() {
        if true == rfl {
            value = table[((value ^ (i as u32)) & 0xFF) as usize] ^ (value >> 8)
        } else {
            value = table[(((value >> shift) as u8) ^ i) as usize] ^ (value << 8);
        }
    }

    value
}

/// Generates a generic IEEE 32 bit CRC (AKA CRC32)
pub fn checksum_ieee(bytes: &[u8]) -> u32 {
    return update(0xFFFFFFFF, &IEEE_TABLE, bytes, true) ^ 0xFFFFFFFF;
}

/// Generates a generic Castagnoli 32 bit CRC (AKA CRC32-C)
pub fn checksum_castagnoli(bytes: &[u8]) -> u32 {
    return update(0xFFFFFFFF, &CASTAGNOLI_TABLE, bytes, true) ^ 0xFFFFFFFF;
}

/// Generates a generic Koopman 32 bit CRC (AKA CRC32-K)
pub fn checksum_koopman(bytes: &[u8]) -> u32 {
    return update(0xFFFFFFFF, &KOOPMAN_TABLE, bytes, true) ^ 0xFFFFFFFF;
}

impl Digest {
    /// Creates a new table from the supplied polynomial and reflect parameter
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
            initial: 0xFFFFFFFF,
            value: 0xFFFFFFFF,
            reflect: true,
            final_xor: 0xFFFFFFFF,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, and an initial value
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc32, Hasher32};
    /// let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, 0xFFFFFFFF);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum32(), 0xcbf43926);
    /// ```
    pub fn new_with_initial(poly: u32, initial: u32) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: initial,
            value: initial,
            reflect: true,
            final_xor: 0xFFFFFFFF,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, initial value, and final XOR value
    ///
    /// This should be the dafault way to generate a custom CRC32.  See default values here: *http://crccalc.com/*
    /// The example will generate a standard CRC32 table.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc32, Hasher32};
    /// let mut digest = crc32::Digest::new_with_initial_and_final(crc32::IEEE, 0xFFFFFFFF, true, 0xFFFFFFFF);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum32(), 0xcbf43926);
    /// ```
    pub fn new_with_initial_and_final(
        poly: u32,
        initial: u32,
        reflect: bool,
        final_xor: u32,
    ) -> Digest {
        Digest {
            table: make_table(poly, reflect),
            initial: initial,
            value: initial,
            reflect: reflect,
            final_xor: final_xor,
        }
    }
}

impl Hasher32 for Digest {
    /// Resets the current CRC to the initial value
    fn reset(&mut self) {
        self.value = self.initial;
    }

    /// Takes in a byte array and updates the CRC from based on the Digest::reflect field
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, self.reflect);
    }

    /// Returns the current CRC after being XOR'd with the final XOR value
    fn sum32(&self) -> u32 {
        self.value ^ self.final_xor
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
