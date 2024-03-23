use crate::util::crc8;
use crc_catalog::Algorithm;

mod bytewise;
mod default;
mod nolookup;
#[cfg(all(
    target_feature = "sse2",
    target_feature = "sse4.1",
    target_feature = "pclmulqdq"
))]
mod simd;
mod slice16;

const fn init(algorithm: &Algorithm<u8>, initial: u8) -> u8 {
    if algorithm.refin {
        initial.reverse_bits() >> (8u8 - algorithm.width)
    } else {
        initial << (8u8 - algorithm.width)
    }
}

const fn finalize(algorithm: &Algorithm<u8>, mut crc: u8) -> u8 {
    if algorithm.refin ^ algorithm.refout {
        crc = crc.reverse_bits();
    }
    if !algorithm.refout {
        crc >>= 8u8 - algorithm.width;
    }
    crc ^ algorithm.xorout
}

const fn update_nolookup(mut crc: u8, algorithm: &Algorithm<u8>, bytes: &[u8]) -> u8 {
    let poly = if algorithm.refin {
        let poly = algorithm.poly.reverse_bits();
        poly >> (8u8 - algorithm.width)
    } else {
        algorithm.poly << (8u8 - algorithm.width)
    };

    let mut i = 0;

    while i < bytes.len() {
        crc = crc8(poly, algorithm.refin, crc ^ bytes[i]);
        i += 1;
    }

    crc
}

const fn update_bytewise(mut crc: u8, table: &[u8; 256], bytes: &[u8]) -> u8 {
    let mut i = 0;

    while i < bytes.len() {
        crc = table[(crc ^ bytes[i]) as usize];
        i += 1;
    }

    crc
}

const fn update_slice16(mut crc: u8, table: &[[u8; 256]; 16], bytes: &[u8]) -> u8 {
    let len = bytes.len();
    let mut i = 0;

    while i + 16 <= len {
        crc = table[0][bytes[i + 15] as usize]
            ^ table[1][bytes[i + 14] as usize]
            ^ table[2][bytes[i + 13] as usize]
            ^ table[3][bytes[i + 12] as usize]
            ^ table[4][bytes[i + 11] as usize]
            ^ table[5][bytes[i + 10] as usize]
            ^ table[6][bytes[i + 9] as usize]
            ^ table[7][bytes[i + 8] as usize]
            ^ table[8][bytes[i + 7] as usize]
            ^ table[9][bytes[i + 6] as usize]
            ^ table[10][bytes[i + 5] as usize]
            ^ table[11][bytes[i + 4] as usize]
            ^ table[12][bytes[i + 3] as usize]
            ^ table[13][bytes[i + 2] as usize]
            ^ table[14][bytes[i + 1] as usize]
            ^ table[15][(bytes[i] ^ crc) as usize];

        i += 16;
    }

    while i < len {
        crc = table[0][(crc ^ bytes[i]) as usize];
        i += 1;
    }

    crc
}

#[cfg(test)]
mod test {
    use crate::{Bytewise, Crc, Implementation, NoTable, Simd, Slice16};
    use crc_catalog::*;

    #[test]
    fn default_table_size() {
        const TABLE_SIZE: usize = core::mem::size_of::<<u8 as Implementation>::Table>();
        const BYTES_PER_ENTRY: usize = 1;
        #[cfg(all(
            feature = "no-table-mem-limit",
            feature = "bytewise-mem-limit",
            feature = "slice16-mem-limit"
        ))]
        {
            const EXPECTED: usize = 0;
            let _ = EXPECTED;
            const _: () = assert!(EXPECTED == TABLE_SIZE);
        }
        #[cfg(all(
            feature = "no-table-mem-limit",
            feature = "bytewise-mem-limit",
            not(feature = "slice16-mem-limit")
        ))]
        {
            const EXPECTED: usize = 0;
            let _ = EXPECTED;
            const _: () = assert!(EXPECTED == TABLE_SIZE);
        }
        #[cfg(all(
            feature = "no-table-mem-limit",
            not(feature = "bytewise-mem-limit"),
            feature = "slice16-mem-limit"
        ))]
        {
            const EXPECTED: usize = 0;
            let _ = EXPECTED;
            const _: () = assert!(EXPECTED == TABLE_SIZE);
        }
        #[cfg(all(
            feature = "no-table-mem-limit",
            not(feature = "bytewise-mem-limit"),
            not(feature = "slice16-mem-limit")
        ))]
        {
            const EXPECTED: usize = 0;
            let _ = EXPECTED;
            const _: () = assert!(EXPECTED == TABLE_SIZE);
        }

        #[cfg(all(
            not(feature = "no-table-mem-limit"),
            feature = "bytewise-mem-limit",
            feature = "slice16-mem-limit"
        ))]
        {
            const EXPECTED: usize = 256 * BYTES_PER_ENTRY;
            let _ = EXPECTED;
            const _: () = assert!(EXPECTED == TABLE_SIZE);
        }
        #[cfg(all(
            not(feature = "no-table-mem-limit"),
            feature = "bytewise-mem-limit",
            not(feature = "slice16-mem-limit")
        ))]
        {
            const EXPECTED: usize = 256 * BYTES_PER_ENTRY;
            let _ = EXPECTED;
            const _: () = assert!(EXPECTED == TABLE_SIZE);
        }

        #[cfg(all(
            not(feature = "no-table-mem-limit"),
            not(feature = "bytewise-mem-limit"),
            feature = "slice16-mem-limit"
        ))]
        {
            const EXPECTED: usize = 256 * 16 * BYTES_PER_ENTRY;
            let _ = EXPECTED;
            const _: () = assert!(EXPECTED == TABLE_SIZE);
        }

        #[cfg(all(
            not(feature = "no-table-mem-limit"),
            not(feature = "bytewise-mem-limit"),
            not(feature = "slice16-mem-limit")
        ))]
        {
            const EXPECTED: usize = 256 * BYTES_PER_ENTRY;
            let _ = EXPECTED;
            const _: () = assert!(EXPECTED == TABLE_SIZE);
        }
        let _ = TABLE_SIZE;
        let _ = BYTES_PER_ENTRY;
    }

    /// Test this optimized version against the well known implementation to ensure correctness
    #[test]
    fn correctness() {
        let data: &[&str] = &[
            "",
            "1",
            "1234",
            "123456789",
            "0123456789ABCDE",
            "01234567890ABCDEFGHIJK",
            "01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK",
        ];

        let algs_to_test = &[
            CRC_3_GSM,
            CRC_3_ROHC,
            CRC_4_G_704,
            CRC_4_INTERLAKEN,
            CRC_5_EPC_C1G2,
            CRC_5_G_704,
            CRC_5_USB,
            CRC_6_CDMA2000_A,
            CRC_6_CDMA2000_B,
            CRC_6_DARC,
            CRC_6_G_704,
            CRC_6_GSM,
            CRC_7_MMC,
            CRC_7_ROHC,
            CRC_7_UMTS,
            CRC_8_AUTOSAR,
            CRC_8_BLUETOOTH,
            CRC_8_CDMA2000,
            CRC_8_DARC,
            CRC_8_DVB_S2,
            CRC_8_GSM_A,
            CRC_8_GSM_B,
            CRC_8_I_432_1,
            CRC_8_I_CODE,
            CRC_8_LTE,
            CRC_8_MAXIM_DOW,
            CRC_8_MIFARE_MAD,
            CRC_8_NRSC_5,
            CRC_8_OPENSAFETY,
            CRC_8_ROHC,
            CRC_8_SAE_J1850,
            CRC_8_SMBUS,
            CRC_8_TECH_3250,
            CRC_8_WCDMA,
        ];

        // Check if the baseline is as expected.
        for alg in algs_to_test {
            assert_eq!(
                Crc::<Bytewise<u8>>::new(alg).checksum("123456789".as_bytes()),
                alg.check
            );
        }

        for alg in algs_to_test {
            for data in data {
                let crc_slice16 = Crc::<Slice16<u8>>::new(alg);
                let crc_nolookup = Crc::<NoTable<u8>>::new(alg);
                let crc_simd: Crc<Simd<u8>> = Crc::<Simd<u8>>::new(alg);
                let expected = Crc::<Bytewise<u8>>::new(alg).checksum(data.as_bytes());

                // Check that doing all at once works as expected
                assert_eq!(crc_slice16.checksum(data.as_bytes()), expected);
                assert_eq!(crc_nolookup.checksum(data.as_bytes()), expected);
                assert_eq!(crc_simd.checksum(data.as_bytes()), expected);

                let mut digest = crc_slice16.digest();
                digest.update(data.as_bytes());
                assert_eq!(digest.finalize(), expected);

                let mut digest = crc_nolookup.digest();
                digest.update(data.as_bytes());
                assert_eq!(digest.finalize(), expected);

                let mut digest = crc_simd.digest();
                digest.update(data.as_bytes());
                assert_eq!(digest.finalize(), expected);

                // Check that we didn't break updating from multiple sources
                if data.len() > 2 {
                    let data = data.as_bytes();
                    let data1 = &data[..data.len() / 2];
                    let data2 = &data[data.len() / 2..];
                    let mut digest = crc_slice16.digest();
                    digest.update(data1);
                    digest.update(data2);
                    assert_eq!(digest.finalize(), expected);
                    let mut digest = crc_nolookup.digest();
                    digest.update(data1);
                    digest.update(data2);
                    assert_eq!(digest.finalize(), expected);
                    let mut digest = crc_simd.digest();
                    digest.update(data1);
                    digest.update(data2);
                    assert_eq!(digest.finalize(), expected);
                }
            }
        }
    }
}
