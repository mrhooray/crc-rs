use crate::util::crc16;
use crc_catalog::Algorithm;

mod bytewise;
mod nolookup;
mod slice16;

#[cfg(all(
    target_feature = "sse2",
    target_feature = "sse4.1",
    target_feature = "pclmulqdq",
))]
mod clmul;

const fn init(algorithm: &Algorithm<u16>, initial: u16) -> u16 {
    if algorithm.refin {
        initial.reverse_bits() >> (16u8 - algorithm.width)
    } else {
        initial << (16u8 - algorithm.width)
    }
}

const fn finalize(algorithm: &Algorithm<u16>, mut crc: u16) -> u16 {
    if algorithm.refin ^ algorithm.refout {
        crc = crc.reverse_bits();
    }
    if !algorithm.refout {
        crc >>= 16u8 - algorithm.width;
    }
    crc ^ algorithm.xorout
}

const fn update_nolookup(mut crc: u16, algorithm: &Algorithm<u16>, bytes: &[u8]) -> u16 {
    let poly = if algorithm.refin {
        let poly = algorithm.poly.reverse_bits();
        poly >> (16u8 - algorithm.width)
    } else {
        algorithm.poly << (16u8 - algorithm.width)
    };

    let mut i = 0;
    if algorithm.refin {
        while i < bytes.len() {
            let to_crc = (crc ^ bytes[i] as u16) & 0xFF;
            crc = crc16(poly, algorithm.refin, to_crc) ^ (crc >> 8);
            i += 1;
        }
    } else {
        while i < bytes.len() {
            let to_crc = ((crc >> 8) ^ bytes[i] as u16) & 0xFF;
            crc = crc16(poly, algorithm.refin, to_crc) ^ (crc << 8);
            i += 1;
        }
    }
    crc
}

const fn update_bytewise(mut crc: u16, reflect: bool, table: &[u16; 256], bytes: &[u8]) -> u16 {
    let mut i = 0;
    if reflect {
        while i < bytes.len() {
            let table_index = ((crc ^ bytes[i] as u16) & 0xFF) as usize;
            crc = table[table_index] ^ (crc >> 8);
            i += 1;
        }
    } else {
        while i < bytes.len() {
            let table_index = (((crc >> 8) ^ bytes[i] as u16) & 0xFF) as usize;
            crc = table[table_index] ^ (crc << 8);
            i += 1;
        }
    }
    crc
}

const fn update_slice16(
    mut crc: u16,
    reflect: bool,
    table: &[[u16; 256]; 16],
    bytes: &[u8],
) -> u16 {
    let len = bytes.len();
    let mut i = 0;
    if reflect {
        while i + 16 <= len {
            let current0 = bytes[i] ^ (crc as u8);
            let current1 = bytes[i + 1] ^ ((crc >> 8) as u8);

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
                ^ table[14][current1 as usize]
                ^ table[15][current0 as usize];

            i += 16;
        }

        while i < len {
            let table_index = ((crc ^ bytes[i] as u16) & 0xFF) as usize;
            crc = table[0][table_index] ^ (crc >> 8);
            i += 1;
        }
    } else {
        while i + 16 <= len {
            let current0 = bytes[i] ^ ((crc >> 8) as u8);
            let current1 = bytes[i + 1] ^ (crc as u8);

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
                ^ table[14][current1 as usize]
                ^ table[15][current0 as usize];

            i += 16;
        }

        while i < len {
            let table_index = (((crc >> 8) ^ bytes[i] as u16) & 0xFF) as usize;
            crc = table[0][table_index] ^ (crc << 8);
            i += 1;
        }
    }
    crc
}

#[cfg(test)]
mod test {
    use crate::*;

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
            CRC_10_ATM,
            CRC_10_CDMA2000,
            CRC_10_GSM,
            CRC_11_FLEXRAY,
            CRC_11_UMTS,
            CRC_12_CDMA2000,
            CRC_12_DECT,
            CRC_12_GSM,
            CRC_12_UMTS,
            CRC_13_BBC,
            CRC_14_DARC,
            CRC_14_GSM,
            CRC_15_CAN,
            CRC_15_MPT1327,
            CRC_16_ARC,
            CRC_16_CDMA2000,
            CRC_16_CMS,
            CRC_16_DDS_110,
            CRC_16_DECT_R,
            CRC_16_DECT_X,
            CRC_16_DNP,
            CRC_16_EN_13757,
            CRC_16_GENIBUS,
            CRC_16_GSM,
            CRC_16_IBM_3740,
            CRC_16_IBM_SDLC,
            CRC_16_ISO_IEC_14443_3_A,
            CRC_16_KERMIT,
            CRC_16_LJ1200,
            CRC_16_MAXIM_DOW,
            CRC_16_MCRF4XX,
            CRC_16_MODBUS,
            CRC_16_NRSC_5,
            CRC_16_OPENSAFETY_A,
            CRC_16_OPENSAFETY_B,
            CRC_16_PROFIBUS,
            CRC_16_RIELLO,
            CRC_16_SPI_FUJITSU,
            CRC_16_T10_DIF,
            CRC_16_TELEDISK,
            CRC_16_TMS37157,
            CRC_16_UMTS,
            CRC_16_USB,
            CRC_16_XMODEM,
        ];

        // Check if the baseline is as expected.
        for alg in algs_to_test {
            assert_eq!(
                Crc::<u16, Table<1>>::new(alg).checksum("123456789".as_bytes()),
                alg.check
            );
        }

        for alg in algs_to_test {
            for data in data {
                let crc_slice16 = Crc::<u16, Table<16>>::new(alg);
                let crc_nolookup = Crc::<u16, NoTable>::new(alg);
                let crc_clmul = Crc::<u16, Clmul>::new(alg);
                let expected = Crc::<u16, Table<1>>::new(alg).checksum(data.as_bytes());

                // Check that doing all at once works as expected
                assert_eq!(crc_slice16.checksum(data.as_bytes()), expected);
                assert_eq!(crc_nolookup.checksum(data.as_bytes()), expected);
                assert_eq!(crc_clmul.checksum(data.as_bytes()), expected);

                let mut digest = crc_slice16.digest();
                digest.update(data.as_bytes());
                assert_eq!(digest.finalize(), expected);

                let mut digest = crc_nolookup.digest();
                digest.update(data.as_bytes());
                assert_eq!(digest.finalize(), expected);

                let mut digest = crc_clmul.digest();
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
                }
            }
        }
    }
}
