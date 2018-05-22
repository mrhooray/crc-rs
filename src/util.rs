/// Builds a CRC16 table using the reverse, reflected method
pub fn make_table_crc16(poly: u16) -> [u16; 256] {
    let mut table = [0u16; 256];
    for i in 0..256 {
        let mut value = i as u16;
        for _ in 0..8 {
            value = if (value & 1) == 1 {
                (value >> 1) ^ poly
            } else {
                value >> 1
            }
        }
        table[i] = value;
    }
    table
}

/// Builds a CRC32 table using the standard CRC method
/// If reflect==true, flip the individual byte bitwise, then flip the 32bit table value bitwise
pub fn make_table_crc32(poly: u32, rfl: bool) -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut byte: u32;
    let top_bit = 1u32 << 31; //31 is 32bit - 1

    for i in 0..256 {
        if true == rfl {
            byte = reflect_byte(i);
        } else {
            byte = i;
        }

        // Shift the cuttent table value "i" to the top byte in the long
        let mut value: u32 = byte << 24; //24=32 bit - 8

        // Step through all the bits in the byte
        for _ in 0..8 {
            if (value & top_bit) != 0 {
                value = (value << 1) ^ poly
            } else {
                value <<= 1
            }
        }

        if true == rfl {
            value = reflect_long(value);
        }

        table[i as usize] = value;
    }
    table
}

/// Builds a CRC64 table using the standard CRC method
/// If reflect==true, flip the individual byte bitwise, then flip the 32bit table value bitwise
pub fn make_table_crc64(poly: u64, rfl: bool) -> [u64; 256] {
    let mut table = [0u64; 256];
    let mut byte: u64;
    let top_bit = 1u64 << 63; //63 is 64bit - 1

    for i in 0..256 {
        if true == rfl {
            byte = reflect_byte_64(i);
        } else {
            byte = i as u64;
        }

        // Shift the cuttent table value "i" to the top byte in the long
        let mut value: u64 = byte << 56; //56=64 bit - 8

        // Step through all the bits in the byte
        for _ in 0..8 {
            if (value & top_bit) != 0 {
                value = (value << 1) ^ poly
            } else {
                value <<= 1
            }
        }

        if true == rfl {
            value = reflect_long_long(value);
        }

        table[i as usize] = value;
    }
    table
}

/// Reflects a value of a 32 bit number
pub fn reflect_long(mut value: u32) -> u32 {
    let mut reflection: u32 = 0u32;
    let bits = 32;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value = value >> 1;
    }
    reflection
}

/// Reflects a value of a 64 bit number
pub fn reflect_long_long(mut value: u64) -> u64 {
    let mut reflection: u64 = 0u64;
    let bits = 64;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value = value >> 1;
    }
    reflection
}

/// Reflects the lease significant byte of a u32.
pub fn reflect_byte(input: u32) -> u32 {
    let mut reflection: u32 = 0u32;
    let bits = 8;
    let mut value = input;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value = value >> 1;
    }
    reflection
}

/// Reflects the lease significant byte of a u64.
pub fn reflect_byte_64(input: u64) -> u64 {
    let mut reflection: u64 = 0u64;
    let bits = 8;
    let mut value = input;

    for i in 0..bits {
        if (value & 0x01) == 1 {
            reflection |= 1 << ((bits - 1) - i)
        }
        value = value >> 1;
    }
    reflection
}
