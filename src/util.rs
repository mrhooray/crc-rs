pub(crate) const fn crc16(poly: u16, reflect: bool, mut byte: u8) -> u16 {
    const fn poly_sum_crc16(poly: u16, value: u16) -> u16 {
        (value << 1) ^ ((value >> 15) * poly)
    }
    byte = [byte, byte.reverse_bits()][reflect as usize];
    let mut value = (byte as u16) << 8;
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    value = poly_sum_crc16(poly, value);
    [value, value.reverse_bits()][reflect as usize]
}

pub(crate) const fn crc32(poly: u32, reflect: bool, mut byte: u8) -> u32 {
    const fn poly_sum_crc32(poly: u32, value: u32) -> u32 {
        (value << 1) ^ ((value >> 31) * poly)
    }
    byte = [byte, byte.reverse_bits()][reflect as usize];
    let mut value = (byte as u32) << 24;
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    value = poly_sum_crc32(poly, value);
    [value, value.reverse_bits()][reflect as usize]
}

pub(crate) const fn crc64(poly: u64, reflect: bool, mut byte: u8) -> u64 {
    const fn poly_sum_crc64(poly: u64, value: u64) -> u64 {
        (value << 1) ^ ((value >> 63) * poly)
    }
    byte = [byte, byte.reverse_bits()][reflect as usize];
    let mut value = (byte as u64) << 56;
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    value = poly_sum_crc64(poly, value);
    [value, value.reverse_bits()][reflect as usize]
}
