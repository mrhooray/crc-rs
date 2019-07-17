/// Builds a CRC16 table using the standard or reflected method.
/// If reflect==true, flip the individual byte bitwise, then flip the table value bitwise.
pub fn make_table_crc16(poly: u16, reflect: bool) -> [u16; 256] {
    let mut table = [0u16; 256];
    let mut byte: u16;
    let top_bit = 1u16 << 15; //15 is 16bit - 1

    for i in 0..256 {
        if reflect {
            byte = reflect_byte_16(i);
        } else {
            byte = i;
        }

        // Shift the current table value "i" to the top byte in the long
        let mut value: u16 = byte << 8; //8=16 bit - 8

        // Step through all the bits in the byte
        for _ in 0..8 {
            if (value & top_bit) != 0 {
                value = (value << 1) ^ poly
            } else {
                value <<= 1
            }
        }

        if reflect {
            value = reflect_value_16(value);
        }

        table[i as usize] = value;
    }
    table
}

/// Builds a CRC32 table using the standard or reflected method.
/// If reflect==true, flip the individual byte bitwise, then flip the table value bitwise.
pub fn make_table_crc32(poly: u32, reflect: bool) -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut byte: u32;
    let top_bit = 1u32 << 31; //31 is 32bit - 1

    for i in 0..256 {
        if reflect {
            byte = reflect_byte_32(i);
        } else {
            byte = i;
        }

        // Shift the current table value "i" to the top byte in the long
        let mut value: u32 = byte << 24; //24=32 bit - 8

        // Step through all the bits in the byte
        for _ in 0..8 {
            if (value & top_bit) != 0 {
                value = (value << 1) ^ poly
            } else {
                value <<= 1
            }
        }

        if reflect {
            value = reflect_value_32(value);
        }

        table[i as usize] = value;
    }
    table
}

/// Builds a CRC64 table using the standard or reflected method.
/// If reflect==true, flip the individual byte bitwise, then flip the table value bitwise.
pub fn make_table_crc64(poly: u64, reflect: bool) -> [u64; 256] {
    let mut table = [0u64; 256];
    let mut byte: u64;
    let top_bit = 1u64 << 63; //63 is 64bit - 1

    for i in 0..256 {
        if reflect {
            byte = reflect_byte_64(i);
        } else {
            byte = i as u64;
        }

        // Shift the current table value "i" to the top byte in the long
        let mut value: u64 = byte << 56; //56=64 bit - 8

        // Step through all the bits in the byte
        for _ in 0..8 {
            if (value & top_bit) != 0 {
                value = (value << 1) ^ poly
            } else {
                value <<= 1
            }
        }

        if reflect {
            value = reflect_value_64(value);
        }

        table[i as usize] = value;
    }
    table
}

/// Reflects a value of a 16 bit number.
fn reflect_value_16(mut value: u16) -> u16 {
    let mut reflection: u16 = 0u16;
    let bits = 16;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value >>= 1;
    }
    reflection
}

/// Reflects a value of a 32 bit number.
fn reflect_value_32(mut value: u32) -> u32 {
    let mut reflection: u32 = 0u32;
    let bits = 32;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value >>= 1;
    }
    reflection
}

/// Reflects a value of a 64 bit number.
fn reflect_value_64(mut value: u64) -> u64 {
    let mut reflection: u64 = 0u64;
    let bits = 64;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value >>= 1;
    }
    reflection
}

/// Reflects the least significant byte of a u16.
fn reflect_byte_16(input: u16) -> u16 {
    let mut reflection: u16 = 0u16;
    let bits = 8;
    let mut value = input;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value >>= 1;
    }
    reflection
}

/// Reflects the least significant byte of a u32.
fn reflect_byte_32(input: u32) -> u32 {
    let mut reflection: u32 = 0u32;
    let bits = 8;
    let mut value = input;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value >>= 1;
    }
    reflection
}

/// Reflects the least significant byte of a u64.
fn reflect_byte_64(input: u64) -> u64 {
    let mut reflection: u64 = 0u64;
    let bits = 8;
    let mut value = input;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value >>= 1;
    }
    reflection
}
