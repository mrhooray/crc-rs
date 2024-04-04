#[cfg(all(
    target_feature = "sse2",
    target_feature = "sse4.1",
    target_feature = "pclmulqdq",
))]
mod x86;

#[cfg(all(
    target_feature = "sse2",
    target_feature = "sse4.1",
    target_feature = "pclmulqdq",
))]
pub(crate) use x86::Value;

trait ValueOps {
    fn xor(self, value: u64) -> Self;

    fn fold_16(self, x_mod_p: Self, value: Self) -> Self;

    fn fold_8(self, x_mod_p: Self) -> Self;

    fn fold_4(self, x_mod_p: Self) -> Self;

    fn barret_reduction_32(self, px_u: Self) -> u32;
}

pub(crate) const fn crc32_coeff(width: u8, poly: u32) -> [Value; 4] {
    const fn xt_mod_px(mut t: u32, px: u64) -> u64 {
        if t < 32 {
            return 0;
        }
        t -= 31;

        let mut n = 0x80000000;
        let mut i = 0;
        while i < t {
            n <<= 1;
            if n & 0x100000000 != 0 {
                n ^= px;
            }
            i += 1;
        }
        n << 32
    }

    const fn u(px: u64) -> u64 {
        let mut q = 0;
        let mut n = 0x100000000;
        let mut i = 0;
        while i < 33 {
            q <<= 1;
            if n & 0x100000000 != 0 {
                q |= 1;
                n ^= px;
            }
            n <<= 1;
            i += 1;
        }
        q
    }

    let px = (poly as u64) << (u32::BITS as u8 - width);
    unsafe {
        // SAFETY: This will be evaluated during compile-time and therefore the alignment
        // doesn't matter, the type is transmuted from 2*u64 to u64x2 simd type.
        core::mem::transmute([
            xt_mod_px(4 * 128 + 32, px).reverse_bits() << 1,
            xt_mod_px(4 * 128 - 32, px).reverse_bits() << 1,
            xt_mod_px(128 + 32, px).reverse_bits() << 1,
            xt_mod_px(128 - 32, px).reverse_bits() << 1,
            xt_mod_px(64, px).reverse_bits() << 1,
            xt_mod_px(32, px).reverse_bits() << 1,
            px.reverse_bits() >> 31,
            u(px).reverse_bits() >> 31,
        ])
    }
}

pub(crate) fn crc32_update_refin(
    crc: u32,
    coeff: &[Value; 4],
    first_chunk: &[Value; 4],
    chunks: &[[Value; 4]],
) -> u32 {
    let mut x4 = *first_chunk;

    // Apply initial crc value
    x4[0] = x4[0].xor(crc as u64);

    // Iteratively Fold by 4:
    let k1_k2 = coeff[0];
    for chunk in chunks {
        for (x, value) in x4.iter_mut().zip(chunk.iter()) {
            *x = x.fold_16(k1_k2, *value)
        }
    }

    // Iteratively Fold by 1:
    let k3_k4 = coeff[1];
    let mut x = x4[0].fold_16(k3_k4, x4[1]);
    x = x.fold_16(k3_k4, x4[2]);
    x = x.fold_16(k3_k4, x4[3]);

    // Final Reduction of 128-bits
    let k5_k6 = coeff[2];
    x = x.fold_8(k3_k4);
    x = x.fold_4(k5_k6);

    // Barret Reduction
    let px_u = coeff[3];
    x.barret_reduction_32(px_u)
}
