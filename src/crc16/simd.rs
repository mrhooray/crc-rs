use crate::crc32::update_simd;
use crate::simd::{SimdValue, SimdValueOps};
use crate::table::{crc16_table_slice_16, crc32_simd_coefficients};
use crate::{Algorithm, Crc, Digest, Simd};

use super::{finalize, init, update_slice16};

impl Crc<Simd<u16>> {
    pub const fn new(algorithm: &'static Algorithm<u16>) -> Self {
        Self {
            algorithm,
            table: (
                crc16_table_slice_16(algorithm.width, algorithm.poly, algorithm.refin),
                unsafe {
                    // SAFETY: SimdValue is the same as u64x2 and this only changes the representation of 8*u64 to 4*u64x2.
                    core::mem::transmute(crc32_simd_coefficients(
                        algorithm.width,
                        algorithm.poly as u32,
                    ))
                },
            ),
        }
    }

    pub fn checksum(&self, bytes: &[u8]) -> u16 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    fn update(&self, mut crc: u16, bytes: &[u8]) -> u16 {
        if !SimdValue::is_supported() || !self.algorithm.refin {
            return update_slice16(crc, self.algorithm.refin, &self.table.0, bytes);
        }

        let (bytes_before, chunks, bytes_after) = unsafe { bytes.align_to::<[SimdValue; 4]>() };
        crc = update_slice16(crc, self.algorithm.refin, &self.table.0, bytes_before);
        if let Some(first_chunk) = chunks.first() {
            // SAFETY: All required features are supported, by checking SimdValue::is_supported.
            crc = unsafe { update_simd(crc as u32, &self.table.1, first_chunk, chunks) } as u16;
        }
        update_slice16(crc, self.algorithm.refin, &self.table.0, bytes_after)
    }

    pub fn digest(&self) -> Digest<Simd<u16>> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub fn digest_with_initial(&self, initial: u16) -> Digest<Simd<u16>> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, Simd<u16>> {
    const fn new(crc: &'a Crc<Simd<u16>>, value: u16) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u16 {
        finalize(self.crc.algorithm, self.value)
    }
}
