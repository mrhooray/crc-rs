use super::{Algorithm, Crc, Digest};
use crate::{table::crc32_table};

/// This implements a faster version of Crc<u32> with a 16kB lookup table
pub struct FastU32;

impl crate::private::Sealed for FastU32 {}

impl crate::Implementation for FastU32 {
    type Width = u32;
    type Table = [[u32; 256]; 16];
}

impl Crc<FastU32> {
    pub const fn new(algorithm: &'static Algorithm<u32>) -> Self {
        let table = crc32_table::<16>(algorithm.width, algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u32 {
        let mut crc = self.init(self.algorithm.init);
        crc = self.update(crc, bytes);
        self.finalize(crc)
    }

    const fn init(&self, initial: u32) -> u32 {
        if self.algorithm.refin {
            initial.reverse_bits() >> (32u8 - self.algorithm.width)
        } else {
            initial << (32u8 - self.algorithm.width)
        }
    }

    const fn update(&self, mut crc: u32, bytes: &[u8]) -> u32 {
        let mut i = 0;
        if self.algorithm.refin {
            while i + 16 <= bytes.len() {
                let mut current_slice = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]];

                current_slice[0] ^= crc as u8;
                current_slice[1] ^= (crc >> 8) as u8;
                current_slice[2] ^= (crc >> 16) as u8;
                current_slice[3] ^= (crc >> 24) as u8;

                crc = self.table[0][bytes[i + 15] as usize]
                    ^ self.table[1][bytes[i + 14] as usize]
                    ^ self.table[2][bytes[i + 13] as usize]
                    ^ self.table[3][bytes[i + 12] as usize]
                    ^ self.table[4][bytes[i + 11] as usize]
                    ^ self.table[5][bytes[i + 10] as usize]
                    ^ self.table[6][bytes[i + 9] as usize]
                    ^ self.table[7][bytes[i + 8] as usize]
                    ^ self.table[8][bytes[i + 7] as usize]
                    ^ self.table[9][bytes[i + 6] as usize]
                    ^ self.table[10][bytes[i + 5] as usize]
                    ^ self.table[11][bytes[i + 4] as usize]
                    ^ self.table[12][current_slice[3] as usize]
                    ^ self.table[13][current_slice[2] as usize]
                    ^ self.table[14][current_slice[1] as usize]
                    ^ self.table[15][current_slice[0] as usize];

                i += 16;
            }

            // Last few bytes
            while i < bytes.len() {
                let table_index = ((crc ^ bytes[i] as u32) & 0xFF) as usize;
                crc = self.table[0][table_index] ^ (crc >> 8);
                i += 1;
            }
        } else {
            while i + 16 <= bytes.len() {
                let mut current_slice = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]];

                current_slice[0] ^= (crc >> 24) as u8;
                current_slice[1] ^= (crc >> 16) as u8;
                current_slice[2] ^= (crc >> 8) as u8;
                current_slice[3] ^= crc as u8;

                crc = self.table[0][bytes[i + 15] as usize]
                    ^ self.table[1][bytes[i + 14] as usize]
                    ^ self.table[2][bytes[i + 13] as usize]
                    ^ self.table[3][bytes[i + 12] as usize]
                    ^ self.table[4][bytes[i + 11] as usize]
                    ^ self.table[5][bytes[i + 10] as usize]
                    ^ self.table[6][bytes[i + 9] as usize]
                    ^ self.table[7][bytes[i + 8] as usize]
                    ^ self.table[8][bytes[i + 7] as usize]
                    ^ self.table[9][bytes[i + 6] as usize]
                    ^ self.table[10][bytes[i + 5] as usize]
                    ^ self.table[11][bytes[i + 4] as usize]
                    ^ self.table[12][current_slice[3] as usize]
                    ^ self.table[13][current_slice[2] as usize]
                    ^ self.table[14][current_slice[1] as usize]
                    ^ self.table[15][current_slice[0] as usize];

                i += 16;
            }

            // Last few bytes
            while i < bytes.len() {
                let table_index = (((crc >> 24) ^ bytes[i] as u32) & 0xFF) as usize;
                crc = self.table[0][table_index] ^ (crc << 8);
                i += 1;
            }
        }
        crc
    }

    const fn finalize(&self, mut crc: u32) -> u32 {
        if self.algorithm.refin ^ self.algorithm.refout {
            crc = crc.reverse_bits();
        }
        if !self.algorithm.refout {
            crc >>= 32u8 - self.algorithm.width;
        }
        crc ^ self.algorithm.xorout
    }

    pub const fn digest(&self) -> Digest<FastU32> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u32) -> Digest<FastU32> {
        let value = self.init(initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, FastU32> {
    const fn new(crc: &'a Crc<FastU32>, value: u32) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u32 {
        self.crc.finalize(self.value)
    }
}

/// Test this opitimized version against the well known implementation to ensure correctness
#[test]
fn correctness() {
    use crc_catalog::CRC_32_ISCSI;

    let data: &[&str] = &[
        "",
        "1",
        "1234",
        "123456789",
        "0123456789ABCDE",
        "01234567890ABCDEFGHIJK",
        "01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK01234567890ABCDEFGHIJK",
    ];

    pub const CRC_32_ISCSI_NONREFLEX: Algorithm<u32> = Algorithm {
        width: 32,
        poly: 0x1edc6f41,
        init: 0xffffffff,
        // This is the only flag that affects the optimized code path
        refin: false,
        refout: true,
        xorout: 0xffffffff,
        check: 0xe3069283,
        residue: 0xb798b438,
    };

    let iscsi = Crc::<FastU32>::new(&CRC_32_ISCSI);
    let iscsi_nonreflex = Crc::<FastU32>::new(&CRC_32_ISCSI_NONREFLEX);

    for data in data {
        let expected = Crc::<u32>::new(&CRC_32_ISCSI).checksum(data.as_bytes());

        // Check that doing all at once works as expected
        let crc1 = iscsi.checksum(data.as_bytes());
        assert_eq!(crc1, expected);

        let mut digest = iscsi.digest();
        digest.update(data.as_bytes());
        let crc2 = digest.finalize();
        assert_eq!(crc2, expected);

        // Check that we didn't break updating from multiple sources
        if data.len() > 2 {
            let data = data.as_bytes();
            let data1 = &data[..data.len() / 2];
            let data2 = &data[data.len() / 2..];
            let mut digest = iscsi.digest();
            digest.update(data1);
            digest.update(data2);
            let crc3 = digest.finalize();
            assert_eq!(crc3, expected);
        }

        let expected = Crc::<u32>::new(&CRC_32_ISCSI_NONREFLEX).checksum(data.as_bytes());

        // Check that doing all at once works as expected
        let crc1 = iscsi_nonreflex.checksum(data.as_bytes());
        assert_eq!(crc1, expected);

        let mut digest = iscsi_nonreflex.digest();
        digest.update(data.as_bytes());
        let crc2 = digest.finalize();
        assert_eq!(crc2, expected);

        // Check that we didn't break updating from multiple sources
        if data.len() > 2 {
            let data = data.as_bytes();
            let data1 = &data[..data.len() / 2];
            let data2 = &data[data.len() / 2..];
            let mut digest = iscsi_nonreflex.digest();
            digest.update(data1);
            digest.update(data2);
            let crc3 = digest.finalize();
            assert_eq!(crc3, expected);
        }
    }
}
