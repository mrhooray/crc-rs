macro_rules! repeat {
    ($repeat_fn:expr, $input:expr,) => ($repeat_fn($input));
    ($repeat_fn:expr, $input:expr, a $($a:ident)*) => (
        $repeat_fn(repeat!($repeat_fn, $input, $($a)*))
    );
}

/// Builds a CRC16 table using the standard or reflected method.
/// If reflect==true, flip the individual byte bitwise, then flip the table value bitwise.
pub const fn make_table_crc16(poly: u16, reflect: bool) -> [u16; 256] {
    let table = [0u16; 256];
    let (_, _, _, table) = repeat!(
        build_table_16,
        (poly, reflect, 0, table),
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
    );
    table
}

/// Builds a CRC32 table using the standard or reflected method.
pub const fn make_table_crc32(poly: u32, reflect: bool) -> [u32; 256] {
    let table = [0u32; 256];
    let (_, _, _, table) = repeat!(
        build_table_32,
        (poly, reflect, 0, table),
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
    );
    table
}

/// Builds a CRC64 table using the standard or reflected method.
pub const fn make_table_crc64(poly: u64, reflect: bool) -> [u64; 256] {
    let table = [0u64; 256];
    let (_, _, _, table) = repeat!(
        build_table_64,
        (poly, reflect, 0, table),
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
    );
    table
}

pub const fn build_table_16((poly, reflect, i, mut table): (u16, bool, u8, [u16; 256])) -> (u16, bool, u8, [u16; 256]) {
    let byte = [i, reflect_byte(i)][reflect as usize] as u16;

    // Shift the current table value "i" to the top byte in the long
    let value: u16 = byte << 8;
    let (_, mut value) = repeat!(
        poly_sum_16,
        (poly, value),
        a a a a a a a
    );

    value = [value, reflect_value_16(value)][reflect as usize];
    table[i as usize] = value;
    (poly, reflect, i + 1, table)
}

pub const fn build_table_32((poly, reflect, i, mut table): (u32, bool, u8, [u32; 256])) -> (u32, bool, u8, [u32; 256]) {
    let byte = [i, reflect_byte(i)][reflect as usize] as u32;

    // Shift the current table value "i" to the top byte in the long
    let value: u32 = byte << 24;
    let (_, mut value) = repeat!(
        poly_sum_32,
        (poly, value),
        a a a a a a a
    );

    value = [value, reflect_value_32(value)][reflect as usize];
    table[i as usize] = value;
    (poly, reflect, i + 1, table)
}

pub const fn build_table_64((poly, reflect, i, mut table): (u64, bool, u8, [u64; 256])) -> (u64, bool, u8, [u64; 256]) {
    let byte = [i, reflect_byte(i)][reflect as usize] as u64;

    // Shift the current table value "i" to the top byte in the long
    let value: u64 = byte << 56;
    let (_, mut value) = repeat!(
        poly_sum_64,
        (poly, value),
        a a a a a a a
    );

    value = [value, reflect_value_64(value)][reflect as usize];
    table[i as usize] = value;
    (poly, reflect, i + 1, table)
}

pub const fn poly_sum_16((poly, value): (u16, u16)) -> (u16, u16) {
    (poly, (value << 1) ^ ((value >> 15) * poly))
}

pub const fn poly_sum_32((poly, value): (u32, u32)) -> (u32, u32) {
    (poly, (value << 1) ^ ((value >> 31) * poly))
}

pub const fn poly_sum_64((poly, value): (u64, u64)) -> (u64, u64) {
    (poly, (value << 1) ^ ((value >> 63) * poly))
}

const fn reflect_byte(mut b: u8) -> u8 {
    b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    b = (b & 0xAA) >> 1 | (b & 0x55) << 1;
    b
}

/// Reflects a value of a 16 bit number.
const fn reflect_value_16(value: u16) -> u16 {
    let l = (reflect_byte(value as u8) as u16) << 8;
    let r = reflect_byte((value >> 8) as u8) as u16;
    l | r
}

/// Reflects a value of a 32 bit number.
const fn reflect_value_32(value: u32) -> u32 {
    let l = (reflect_value_16(value as u16) as u32) << 16;
    let r = reflect_value_16((value >> 16) as u16) as u32;
    l | r
}

/// Reflects a value of a 64 bit number.
const fn reflect_value_64(value: u64) -> u64 {
    let l = (reflect_value_32(value as u32) as u64) << 32;
    let r = reflect_value_32((value >> 32) as u32) as u64;
    l | r
}
