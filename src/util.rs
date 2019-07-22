#[doc(hidden)]
macro_rules! reflect {
    ($bits:expr, $value:expr) => {{
        let mut reflection = 0;
        let mut value = $value;

        for i in 0..$bits {
            if (value & 0x01) == 1 {
                reflection |= 1 << (($bits - 1) - i)
            }

            value >>= 1;
        }

        reflection
    }};
}

#[doc(hidden)]
macro_rules! reflect_byte {
    ($value:expr) => {{
        reflect!(8, $value)
    }};
}

#[doc(hidden)]
macro_rules! reflect_value {
    ($value:expr) => {{
        reflect!(core::mem::size_of_val(&$value) * 8, $value)
    }};
}

macro_rules! make_table {
    ($polynomial:expr, $reflect_in:expr, $reflect_out:expr) => {{
        let mut table = [0; 256];
        let bits = core::mem::size_of_val(&$polynomial) * 8;
        let top_bit = 1 << (bits - 1);
        let mut byte;

        for i in 0..256 {
            if $reflect_in {
                byte = reflect_byte!(i);
            } else {
                byte = i;
            }

            // Shift the current table value "i" to the top byte in the long
            let mut value = byte << (bits - 8);

            // Step through all the bits in the byte
            for _ in 0..8 {
                if (value & top_bit) != 0 {
                    value = (value << 1) ^ $polynomial
                } else {
                    value <<= 1
                }
            }

            if $reflect_out {
                value = reflect_value!(value);
            }

            table[i as usize] = value;
        }

        table
    }};
}

/// Builds a CRC16 table using the standard or reflected method.
/// If reflect==true, flip the individual byte bitwise, then flip the table value bitwise.
pub fn make_table_crc16(poly: u16, reflect: bool) -> [u16; 256] {
    make_table!(poly, reflect, reflect)
}

/// Builds a CRC32 table using the standard or reflected method.
/// If reflect==true, flip the individual byte bitwise, then flip the table value bitwise.
pub fn make_table_crc32(poly: u32, reflect: bool) -> [u32; 256] {
    make_table!(poly, reflect, reflect)
}

/// Builds a CRC64 table using the standard or reflected method.
/// If reflect==true, flip the individual byte bitwise, then flip the table value bitwise.
pub fn make_table_crc64(poly: u64, reflect: bool) -> [u64; 256] {
    make_table!(poly, reflect, reflect)
}
