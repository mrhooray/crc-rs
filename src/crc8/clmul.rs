use crate::crc8::{finalize, init, update_bytewise};
use crate::*;
use crate::{clmul::crc32_clmul_coeff, table::crc8_table};

use self::clmul::{crc32_update_refin, Value};

impl Crc<u8, Clmul> {
    pub const fn new(algorithm: &'static Algorithm<u8>) -> Self {
        let table = crc8_table(algorithm.width, algorithm.poly, algorithm.refin);
        let coeff = crc32_clmul_coeff(algorithm.width, algorithm.poly as u32);
        Self {
            algorithm,
            data: (table, coeff),
        }
    }

    pub fn checksum(&self, bytes: &[u8]) -> u8 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    fn update(&self, mut crc: u8, bytes: &[u8]) -> u8 {
        if !self.algorithm.refin {
            return update_bytewise(crc, &self.data.0, bytes);
        }

        // SAFETY: The returned value for chunks will always be aligned,
        // considering the platform requirement and 64*8-bit chunks are transmuted
        // to 4*128-bit chunks and the lifetime and mutability does not change.
        let (bytes_before, chunks, bytes_after) = unsafe { bytes.align_to::<[Value; 4]>() };
        crc = update_bytewise(crc, &self.data.0, bytes_before);
        if let Some(first_chunk) = chunks.first() {
            crc = crc32_update_refin(crc as u32, &self.data.1, first_chunk, &chunks[1..]) as u8;
        }
        update_bytewise(crc, &self.data.0, bytes_after)
    }

    pub const fn digest(&self) -> Digest<u8, Clmul> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u8) -> Digest<u8, Clmul> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, u8, Clmul> {
    const fn new(crc: &'a Crc<u8, Clmul>, value: u8) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u8 {
        finalize(self.crc.algorithm, self.value)
    }
}
