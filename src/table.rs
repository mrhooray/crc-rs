use crate::util::*;

macro_rules! impl_crc_table {
    ($name: ident, $ty: ty, $base: path) => {
        pub(crate) const fn $name(width: u8, poly: $ty, reflect: bool) -> [$ty; 256] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;

            let poly = if reflect {
                let poly = poly.reverse_bits();
                poly >> (BITS - width as usize)
            } else {
                poly << (BITS - width as usize)
            };

            let mut table = [0 as $ty; 256];
            let mut i = 0;
            while i < table.len() {
                table[i] = $base(poly, reflect, i as $ty);
                i += 1;
            }
            table
        }
    };
}
macro_rules! impl_crc_slice_table {
    ($name: ident, $ty: ty, $slices: expr, $base: path) => {
        pub(crate) const fn $name(width: u8, poly: $ty, reflect: bool) -> [[$ty; 256]; $slices] {
            const BITS: usize = ::core::mem::size_of::<$ty>() * 8;
            // to avoid compiler arithmetic_overflow error
            const SHIFT: usize = if BITS > 8 { 8 } else { 0 };

            let poly = if reflect {
                let poly = poly.reverse_bits();
                poly >> (BITS - width as usize)
            } else {
                poly << (BITS - width as usize)
            };

            let mut table = [[0 as $ty; 256]; $slices];
            let mut i = 0;
            while i < 256 {
                table[0][i] = $base(poly, reflect, i as $ty);
                i += 1;
            }

            let mut i = 0;
            while i < 256 {
                let mut e = 1;
                while e < $slices {
                    let one_lower = table[e - 1][i];

                    if reflect {
                        table[e][i] = table[0][(one_lower & 0xFF) as usize];

                        if BITS > 8 {
                            table[e][i] ^= (one_lower >> SHIFT);
                        }
                    } else {
                        table[e][i] = table[0][((one_lower >> (BITS - 8)) & 0xFF) as usize];

                        if BITS > 8 {
                            table[e][i] ^= (one_lower << SHIFT);
                        }
                    }
                    e += 1;
                }
                i += 1;
            }
            table
        }
    };
}

impl_crc_table!(crc8_table, u8, crc8);
impl_crc_table!(crc16_table, u16, crc16);
impl_crc_table!(crc32_table, u32, crc32);
impl_crc_table!(crc64_table, u64, crc64);
impl_crc_table!(crc128_table, u128, crc128);

impl_crc_slice_table!(crc8_table_slice_16, u8, 16, crc8);
impl_crc_slice_table!(crc16_table_slice_16, u16, 16, crc16);
impl_crc_slice_table!(crc32_table_slice_16, u32, 16, crc32);
impl_crc_slice_table!(crc64_table_slice_16, u64, 16, crc64);
impl_crc_slice_table!(crc128_table_slice_16, u128, 16, crc128);
