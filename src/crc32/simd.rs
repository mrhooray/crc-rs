use crate::simd::{crc32_simd_coefficients, crc32_update_refin, SimdValue};
use crate::table::crc32_table_slice_16;
use crate::{Algorithm, Crc, Digest, Simd};

use super::{finalize, init, update_slice16};

impl Crc<Simd<u32>> {
    pub const fn new(algorithm: &'static Algorithm<u32>) -> Self {
        Self {
            algorithm,
            table: (
                crc32_table_slice_16(algorithm.width, algorithm.poly, algorithm.refin),
                unsafe {
                    // SAFETY: Both represent numbers.
                    core::mem::transmute(crc32_simd_coefficients(algorithm.width, algorithm.poly))
                },
            ),
        }
    }

    pub fn checksum(&self, bytes: &[u8]) -> u32 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    fn update(&self, mut crc: u32, bytes: &[u8]) -> u32 {
        if !self.algorithm.refin {
            return update_slice16(crc, self.algorithm.refin, &self.table.0, bytes);
        }

        // SAFETY: Both represent numbers.
        let (bytes_before, chunks, bytes_after) = unsafe { bytes.align_to::<[SimdValue; 4]>() };
        crc = update_slice16(crc, self.algorithm.refin, &self.table.0, bytes_before);
        if let Some(first_chunk) = chunks.first() {
            // SAFETY: All features are supported as the program has been compiled with all required target features set.
            crc = unsafe { crc32_update_refin(crc, &self.table.1, first_chunk, &chunks[1..]) };
        }
        update_slice16(crc, self.algorithm.refin, &self.table.0, bytes_after)
    }

    pub fn digest(&self) -> Digest<Simd<u32>> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub fn digest_with_initial(&self, initial: u32) -> Digest<Simd<u32>> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, Simd<u32>> {
    const fn new(crc: &'a Crc<Simd<u32>>, value: u32) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u32 {
        finalize(self.crc.algorithm, self.value)
    }
}
