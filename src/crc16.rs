#[cfg(not(feature = "std"))]
use core::hash::Hasher;
#[cfg(feature = "std")]
use std::hash::Hasher;

pub use util::make_table_crc16 as make_table;
pub use util::CalcType;

include!(concat!(env!("OUT_DIR"), "/crc16_constants.rs"));

/// Structure that holds all of the important values for calculating a CRC.
/// ### Details
/// - **table**: Holds the table values based on the supplied polynomial for the fast CRC calculations
/// - **initial**: The initial input value. AKA *reflect_in*
/// - **value**: Holds the current value of the CRC
/// - **reflect**: Chooses whether or not the CRC math is *Normal* or *Reverse*
/// - **final_xor**: Final value to XOR with when calling `Digest::sum16()`. AKA *reflect_out*
pub struct Digest {
    table: [u16; 256],
    initial: u16,
    value: u16,
    reflect: CalcType,
    final_xor: u16,
}

pub trait Hasher16 {
    fn reset(&mut self);
    fn write(&mut self, bytes: &[u8]);
    fn sum16(&self) -> u16;
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
/// call using `Digest::write(&bytes)`
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

/// Generates a generic x25 16 bit CRC (AKA CRC-16-CCITT).
pub fn checksum_x25(bytes: &[u8]) -> u16 {
    return update(0u16, &X25_TABLE, bytes, &CalcType::Compat);
}

/// Generates a generic USB 16 bit CRC (AKA CRC-16-IBM).
pub fn checksum_usb(bytes: &[u8]) -> u16 {
    return update(0u16, &USB_TABLE, bytes, &CalcType::Compat);
}

impl Digest {
    /// Creates a new table from the supplied polynomial and reflect parameter.
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
            reflect: CalcType::Compat,
            final_xor: 0u16,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, and an initial value.
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
            reflect: CalcType::Compat,
            final_xor: 0u16,
        }
    }

    /// Creates a new table from the supplied polynomial, reflect parameter, initial value, and final XOR value.
    /// ### Details
    /// This should be the default way to generate a custom CRC16.  See default values here: *http://crccalc.com/*.
    /// The example will generate a standard CRC16 table.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crc::{crc16, Hasher16};
    /// let mut digest = crc16::Digest::new_custom(crc16::X25, 0xFFFF, crc16::CalcType::Reverse, 0xFFFF);
    /// digest.write(b"123456789");
    /// assert_eq!(digest.sum16(), 0x906e);
    /// ```
    pub fn new_custom(poly: u16, initial: u16, reflect: CalcType, final_xor: u16) -> Digest {
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

impl Hasher16 for Digest {
    /// Resets the current CRC in *value* to the *initial* value
    fn reset(&mut self) {
        self.value = self.initial;
    }

    /// Takes in a byte array and updates the CRC from based on the `Digest::reflect` field
    fn write(&mut self, bytes: &[u8]) {
        self.value = update(self.value, &self.table, bytes, &self.reflect);
    }

    /// Returns the current CRC after being XOR'd with the final XOR value
    fn sum16(&self) -> u16 {
        self.value ^ self.final_xor
    }
}

/// Implementation of `std::hash::Hasher` so that types which #[derive(Hash)] can hash with Digest.
impl Hasher for Digest {
    fn finish(&self) -> u64 {
        self.sum16() as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        Hasher16::write(self, bytes);
    }
}
