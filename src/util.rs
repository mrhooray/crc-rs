macro_rules! repeat {
    ($repeat_fn:expr, $input:expr,) => ($repeat_fn($input));
    ($repeat_fn:expr, $input:expr, a $($a:ident)*) => (
        $repeat_fn(repeat!($repeat_fn, $input, $($a)*))
    );
}

macro_rules! repeat_i {
    ($repeat_fn:expr, $input:expr, $count:expr,) => ($repeat_fn($input, $count));
    ($repeat_fn:expr, $input:expr, $count:expr, a $($a:ident)*) => (
        $repeat_fn(repeat_i!($repeat_fn, $input, $count + 1, $($a)*), $count)
    );
}

/// Builds a CRC16 table using the standard or reflected method.
/// If reflect==true, flip the individual byte bitwise, then flip the table value bitwise.
pub const fn make_table_crc16(poly: u16, reflect: bool) -> [u16; 256] {
    let table = [0u16; 256];
    let (_, _, table) = repeat_i!(
        build_table_16,
        (poly, reflect, table),
        0u8, 
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
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
    let (_, _, table) = repeat_i!(
        build_table_32,
        (poly, reflect, table),
        0u8, 
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
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
    let (_, _, table) = repeat_i!(
        build_table_64,
        (poly, reflect, table),
        0u8, 
        a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a
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

pub const fn build_table_16((poly, reflect, mut table): (u16, bool, [u16; 256]), i: u8) -> (u16, bool, [u16; 256]) {
    let byte = [i, reflect_8(i)][reflect as usize] as u16;

    // Shift the current table value "i" to the top byte in the long
    let value: u16 = byte << 8;
    let (_, mut value) = repeat!(
        poly_sum_16,
        (poly, value),
        a a a a a a a
    );

    value = [value, reflect_16(value)][reflect as usize];
    table[i as usize] = value;
    (poly, reflect, table)
}

pub const fn build_table_32((poly, reflect, mut table): (u32, bool, [u32; 256]), i: u8) -> (u32, bool, [u32; 256]) {
    let byte = [i, reflect_8(i)][reflect as usize] as u32;

    // Shift the current table value "i" to the top byte in the long
    let value: u32 = byte << 24;
    let (_, mut value) = repeat!(
        poly_sum_32,
        (poly, value),
        a a a a a a a
    );

    value = [value, reflect_32(value)][reflect as usize];
    table[i as usize] = value;
    (poly, reflect, table)
}

pub const fn build_table_64((poly, reflect, mut table): (u64, bool, [u64; 256]), i: u8) -> (u64, bool, [u64; 256]) {
    let byte = [i, reflect_8(i)][reflect as usize] as u64;

    // Shift the current table value "i" to the top byte in the long
    let value: u64 = byte << 56;
    let (_, mut value) = repeat!(
        poly_sum_64,
        (poly, value),
        a a a a a a a
    );

    value = [value, reflect_64(value)][reflect as usize];
    table[i as usize] = value;
    (poly, reflect, table)
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

const fn reflect_8(mut b: u8) -> u8 {
    b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    b = (b & 0xAA) >> 1 | (b & 0x55) << 1;
    b
}

const fn reflect_16(mut b: u16) -> u16 {
    b = (b & 0xFF00) >> 8 | (b & 0x00FF) << 8;
    b = (b & 0xF0F0) >> 4 | (b & 0x0F0F) << 4;
    b = (b & 0xCCCC) >> 2 | (b & 0x3333) << 2;
    b = (b & 0xAAAA) >> 1 | (b & 0x5555) << 1;
    b
}

const fn reflect_32(mut b: u32) -> u32 {
    b = (b & 0xFFFF0000) >> 16 | (b & 0x0000FFFF) << 16;
    b = (b & 0xFF00FF00) >> 8  | (b & 0x00FF00FF) << 8;
    b = (b & 0xF0F0F0F0) >> 4  | (b & 0x0F0F0F0F) << 4;
    b = (b & 0xCCCCCCCC) >> 2  | (b & 0x33333333) << 2;
    b = (b & 0xAAAAAAAA) >> 1  | (b & 0x55555555) << 1;
    b
}

const fn reflect_64(mut b: u64) -> u64 {
    b = (b & 0xFFFFFFFF00000000) >> 32 | (b & 0x00000000FFFFFFFF) << 32;
    b = (b & 0xFFFF0000FFFF0000) >> 16 | (b & 0x0000FFFF0000FFFF) << 16;
    b = (b & 0xFF00FF00FF00FF00) >> 8  | (b & 0x00FF00FF00FF00FF) << 8;
    b = (b & 0xF0F0F0F0F0F0F0F0) >> 4  | (b & 0x0F0F0F0F0F0F0F0F) << 4;
    b = (b & 0xCCCCCCCCCCCCCCCC) >> 2  | (b & 0x3333333333333333) << 2;
    b = (b & 0xAAAAAAAAAAAAAAAA) >> 1  | (b & 0x5555555555555555) << 1;
    b
}
