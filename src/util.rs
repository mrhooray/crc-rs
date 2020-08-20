pub(crate) const fn crc16(poly: u16, reflect: bool, mut byte: u8) -> u16 {
    const fn poly_sum_crc16(poly: u16, value: u16) -> u16 {
        (value << 1) ^ ((value >> 15) * poly)
    }
    byte = [byte, reflect_8(byte)][reflect as usize];
    let mut value = (byte as u16) << 8;
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    [value, reflect_16(value)][reflect as usize]
}

pub(crate) const fn crc32(poly: u32, reflect: bool, mut byte: u8) -> u32 {
    const fn poly_sum_crc32(poly: u32, value: u32) -> u32 {
        (value << 1) ^ ((value >> 31) * poly)
    }
    byte = [byte, reflect_8(byte)][reflect as usize];
    let mut value = (byte as u32) << 24;
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    [value, reflect_32(value)][reflect as usize]
}

pub(crate) const fn crc64(poly: u64, reflect: bool, mut byte: u8) -> u64 {
    const fn poly_sum_crc64(poly: u64, value: u64) -> u64 {
        (value << 1) ^ ((value >> 63) * poly)
    }
    byte = [byte, reflect_8(byte)][reflect as usize];
    let mut value = (byte as u64) << 56;
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    [value, reflect_64(value)][reflect as usize]
}

pub(crate) const fn reflect_8(mut b: u8) -> u8 {
    b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    b = (b & 0xAA) >> 1 | (b & 0x55) << 1;
    b
}

pub(crate) const fn reflect_16(mut b: u16) -> u16 {
    b = (b & 0xFF00) >> 8 | (b & 0x00FF) << 8;
    b = (b & 0xF0F0) >> 4 | (b & 0x0F0F) << 4;
    b = (b & 0xCCCC) >> 2 | (b & 0x3333) << 2;
    b = (b & 0xAAAA) >> 1 | (b & 0x5555) << 1;
    b
}

pub(crate) const fn reflect_32(mut b: u32) -> u32 {
    b = (b & 0xFFFF0000) >> 16 | (b & 0x0000FFFF) << 16;
    b = (b & 0xFF00FF00) >> 8 | (b & 0x00FF00FF) << 8;
    b = (b & 0xF0F0F0F0) >> 4 | (b & 0x0F0F0F0F) << 4;
    b = (b & 0xCCCCCCCC) >> 2 | (b & 0x33333333) << 2;
    b = (b & 0xAAAAAAAA) >> 1 | (b & 0x55555555) << 1;
    b
}

pub(crate) const fn reflect_64(mut b: u64) -> u64 {
    b = (b & 0xFFFFFFFF00000000) >> 32 | (b & 0x00000000FFFFFFFF) << 32;
    b = (b & 0xFFFF0000FFFF0000) >> 16 | (b & 0x0000FFFF0000FFFF) << 16;
    b = (b & 0xFF00FF00FF00FF00) >> 8 | (b & 0x00FF00FF00FF00FF) << 8;
    b = (b & 0xF0F0F0F0F0F0F0F0) >> 4 | (b & 0x0F0F0F0F0F0F0F0F) << 4;
    b = (b & 0xCCCCCCCCCCCCCCCC) >> 2 | (b & 0x3333333333333333) << 2;
    b = (b & 0xAAAAAAAAAAAAAAAA) >> 1 | (b & 0x5555555555555555) << 1;
    b
}
