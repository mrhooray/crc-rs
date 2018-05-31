#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

pub use util::make_table_crc16 as make_table;

include!(concat!(env!("OUT_DIR"), "/crc16_constants.rs"));

/// Structure that holds all of the important values for calculating a CRC
///
/// # Definitions
///
/// * **table:** Holds the table values based on the supplied polynomial for the fast CRC calculations
/// * **initial:** The initial input value. AKA *reflect_in*
/// * **value:** Holds the current value of the CRC
/// * **reflect:** Chooses whether or not the CRC math is normal or reflected
/// * **final_xor:** Final value to XOR with when calling Digest::sum16
pub struct Digest {
    table: [u16; 256],
    initial: u16,
    value: u16,
    reflect: bool,
    final_xor: u16,
}

pub trait Hasher16 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum16(&self) -> u16;
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
pub fn update(mut value: u16, table: &[u16; 256], bytes: &[u8], rfl: bool) -> u16 {
    if rfl {
        value = bytes.iter().fold(value, |acc, &x| {
            (acc >> 8) ^ (table[((acc ^ (u16::from(x))) & 0xFF) as usize])
        });
    } else {
        value = bytes.iter().fold(value, |acc, &x| {
            (acc << 8) ^ (table[((u16::from(x)) ^ (acc >> 8)) as usize])
        });
    }
    value
}

/// Generates a generic x25 16 bit CRC (AKA CRC-16-CCITT)
pub fn checksum_x25(bytes: &[u8]) -> u16 {
    return update(0xFFFF, &X25_TABLE, bytes, true) ^ 0xFFFF;
}

/// Generates a generic USB 16 bit CRC (AKA CRC-16-IBM)
pub fn checksum_usb(bytes: &[u8]) -> u16 {
    return update(0xFFFF, &USB_TABLE, bytes, true) ^ 0xFFFF;
}

impl Digest {
    /// Creates a new table from the supplied polynomial and reflect parameter
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
            initial: 0xFFFF,
            value: 0xFFFF,
            reflect: true,
            final_xor: 0xFFFF,
        }
    }

    /// *Only works for reflected CRCs*
    /// Creates a new table from the supplied polynomial, reflect parameter, and an initial value
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc16, Hasher16};
    /// let mut digest = crc16::Digest::new_with_initial(crc16::X25, 0xFFFF);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum16(), 0x906e);
    /// ```
    pub fn new_with_initial(poly: u16, initial: u16) -> Digest {
        Digest {
            table: make_table(poly, true),
            initial: initial,
            value: initial,
            reflect: true,
            final_xor: 0xFFFF,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, initial value, and final XOR value
    ///
    /// This should be the dafault way to generate a custom CRC16.  See default values here: *http://crccalc.com/*
    /// The example will generate a standard CRC16 table.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc16, Hasher16};
    /// let mut digest = crc16::Digest::new_custom(crc16::X25, 0xFFFF, true, 0xFFFF);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum16(), 0x906e);
    /// ```
    pub fn new_custom(poly: u16, initial: u16, reflect: bool, final_xor: u16) -> Digest {
        Digest {
            table: make_table(poly, reflect),
            initial: initial,
            value: initial,
            reflect: reflect,
            final_xor: final_xor,
        }
    }
}

impl Hasher16 for Digest {
    /// Resets the current CRC to the initial value
    fn reset(&mut self) {
        self.value = self.initial;
    }

    /// Takes in a byte array and updates the CRC from based on the Digest::reflect field
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, self.reflect);
    }

    /// Returns the current CRC after being XOR'd with the final XOR value
    fn sum16(&self) -> u16 {
        self.value ^ self.final_xor
    }
}

/// Implementation of std::hash::Hasher so that types which #[derive(Hash)] can hash with Digest.
impl Hasher for Digest {
    fn write(&mut self, bytes: &[u8]) {
        Hasher16::write(self, bytes);
    }

    fn finish(&self) -> u64 {
        self.sum16() as u64
    }
}
