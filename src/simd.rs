mod x86;

pub(crate) trait SimdValueOps {
    fn is_supported() -> bool;

    unsafe fn xor(self, value: u64) -> Self;

    unsafe fn fold_16(self, x_mod_p: Self, value: Self) -> Self;

    unsafe fn fold_8(self, x_mod_p: Self) -> Self;

    unsafe fn fold_4(self, x_mod_p: Self) -> Self;

    unsafe fn barret_reduction_32(self, px_u: Self) -> u32;
}

pub(crate) use x86::SimdValue;
