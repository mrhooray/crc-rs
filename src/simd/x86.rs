use crate::simd::SimdValueOps;

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;
use core::mem;

#[derive(Copy, Clone)]
pub struct SimdValue(arch::__m128i);

impl SimdValueOps for SimdValue {
    fn is_supported() -> bool {
        cfg!(target_feature = "pclmulqdq")
            && cfg!(target_feature = "sse2")
            && cfg!(target_feature = "sse4.1")
    }

    #[inline]
    #[target_feature(enable = "sse2")]
    unsafe fn xor(self, value: u64) -> Self {
        Self(arch::_mm_xor_si128(
            self.0,
            arch::_mm_set_epi64x(0, value as i64),
        ))
    }

    #[inline]
    #[target_feature(enable = "sse2", enable = "pclmulqdq")]
    unsafe fn fold_16(self, x_mod_p: Self, value: Self) -> Self {
        Self(arch::_mm_xor_si128(
            arch::_mm_clmulepi64_si128(self.0, x_mod_p.0, 0x00),
            arch::_mm_xor_si128(arch::_mm_clmulepi64_si128(self.0, x_mod_p.0, 0x11), value.0),
        ))
    }

    #[inline]
    #[target_feature(enable = "sse2", enable = "pclmulqdq")]
    unsafe fn fold_8(self, x_mod_p: Self) -> Self {
        Self(arch::_mm_xor_si128(
            arch::_mm_clmulepi64_si128(self.0, x_mod_p.0, 0x10),
            arch::_mm_srli_si128(self.0, 8),
        ))
    }

    #[inline]
    #[target_feature(enable = "sse2", enable = "pclmulqdq")]
    unsafe fn fold_4(self, x_mod_p: Self) -> Self {
        Self(arch::_mm_xor_si128(
            arch::_mm_clmulepi64_si128(
                arch::_mm_and_si128(self.0, mem::transmute((1u128 << 32) - 1)),
                x_mod_p.0,
                0x00,
            ),
            arch::_mm_srli_si128(self.0, 4),
        ))
    }

    #[inline]
    #[target_feature(enable = "sse2", enable = "sse4.1", enable = "pclmulqdq")]
    unsafe fn barret_reduction_32(self, px_u: Self) -> u32 {
        let t1 = arch::_mm_clmulepi64_si128(
            arch::_mm_and_si128(self.0, mem::transmute((1u128 << 32) - 1)),
            px_u.0,
            0x10,
        );
        let t2 = arch::_mm_clmulepi64_si128(
            arch::_mm_and_si128(t1, mem::transmute((1u128 << 32) - 1)),
            px_u.0,
            0x00,
        );
        arch::_mm_extract_epi32(arch::_mm_xor_si128(self.0, t2), 1) as u32
    }
}
