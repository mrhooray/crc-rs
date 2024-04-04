use crate::*;
use crate::{simd::crc32_coeff, table::crc32_table};

use crate::crc32::{finalize, init, update_bytewise};

use self::simd::{crc32_update_refin, Value};

impl Crc<u32, Simd> {
    pub const fn new(algorithm: &'static Algorithm<u32>) -> Self {
        let table = crc32_table(algorithm.width, algorithm.poly, algorithm.refin);
        let coeff = crc32_coeff(algorithm.width, algorithm.poly);
        Self {
            algorithm,
            data: (table, coeff),
        }
    }

    pub fn checksum(&self, bytes: &[u8]) -> u32 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    fn update(&self, mut crc: u32, bytes: &[u8]) -> u32 {
        if !self.algorithm.refin {
            return update_bytewise(crc, self.algorithm.refin, &self.data.0, bytes);
        }

        // SAFETY: The returned value for chunks will always be aligned,
        // considering the platform requirement and 64*8-bit chunks are transmuted
        // to 4*128-bit chunks and the lifetime and mutability does not change.
        let (bytes_before, chunks, bytes_after) = unsafe { bytes.align_to::<[Value; 4]>() };
        crc = update_bytewise(crc, self.algorithm.refin, &self.data.0, bytes_before);
        if let Some(first_chunk) = chunks.first() {
            crc = crc32_update_refin(crc, &self.data.1, first_chunk, &chunks[1..]);
        }
        update_bytewise(crc, self.algorithm.refin, &self.data.0, bytes_after)
    }

    pub const fn digest(&self) -> Digest<u32, Simd> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u32) -> Digest<u32, Simd> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, u32, Simd> {
    const fn new(crc: &'a Crc<u32, Simd>, value: u32) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u32 {
        finalize(self.crc.algorithm, self.value)
    }
}
