pub(crate) const fn crc8(poly: u8, reflect: bool, mut value: u8) -> u8 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= u8::BITS - 8;

        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> (u8::BITS - 1)) & 1) * poly);
            i += 1;
        }
    }
    value
}

pub(crate) const fn crc16(poly: u16, reflect: bool, mut value: u16) -> u16 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= u16::BITS - 8;

        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> (u16::BITS - 1)) & 1) * poly);
            i += 1;
        }
    }
    value
}

pub(crate) const fn crc32(poly: u32, reflect: bool, mut value: u32) -> u32 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= u32::BITS - 8;

        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> (u32::BITS - 1)) & 1) * poly);
            i += 1;
        }
    }
    value
}

pub(crate) const fn crc64(poly: u64, reflect: bool, mut value: u64) -> u64 {
    if reflect {
        let mut i = 0;
        while i < 8 {
            value = (value >> 1) ^ ((value & 1) * poly);
            i += 1;
        }
    } else {
        value <<= u64::BITS - 8;

        let mut i = 0;
        while i < 8 {
            value = (value << 1) ^ (((value >> (u64::BITS - 1)) & 1) * poly);
            i += 1;
        }
    }
    value
}
