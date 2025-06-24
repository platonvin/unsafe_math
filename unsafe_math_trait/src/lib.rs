//! Helper trait to provide the fast-math operations
//!
//! Originally i wanted to just pick functions based on type inside macro, but they dont have acces to type information.
//! And traits just happen to do exactly whats needed.

#![allow(internal_features)]
#![feature(core_intrinsics)]

/// Helper trait to provide the fast-math operations for all integer and float types.
pub trait UnsafeMath: Sized {
    fn fast_add(self, rhs: Self) -> Self;
    fn fast_sub(self, rhs: Self) -> Self;
    fn fast_mul(self, rhs: Self) -> Self;
    fn fast_div(self, rhs: Self) -> Self;
    fn fast_rem(self, rhs: Self) -> Self;
    fn fast_shl(self, rhs: u32) -> Self;
    fn fast_shr(self, rhs: u32) -> Self;
}

macro_rules! impl_fast_math_for_int {
        ($($t:ty),*) => {
            $(
                impl UnsafeMath for $t {
                    #[inline(always)] fn fast_add(self, rhs: Self) -> Self { unsafe { std::intrinsics::unchecked_add(self, rhs) } }
                    #[inline(always)] fn fast_sub(self, rhs: Self) -> Self { unsafe { std::intrinsics::unchecked_sub(self, rhs) } }
                    #[inline(always)] fn fast_mul(self, rhs: Self) -> Self { unsafe { std::intrinsics::unchecked_mul(self, rhs) } }
                    #[inline(always)] fn fast_div(self, rhs: Self) -> Self { unsafe { std::intrinsics::unchecked_div(self, rhs) } }
                    #[inline(always)] fn fast_rem(self, rhs: Self) -> Self { unsafe { std::intrinsics::unchecked_rem(self, rhs) } }
                    #[inline(always)] fn fast_shl(self, rhs: u32) -> Self { unsafe { std::intrinsics::unchecked_shl(self, rhs) } }
                    #[inline(always)] fn fast_shr(self, rhs: u32) -> Self { unsafe { std::intrinsics::unchecked_shr(self, rhs) } }
                }
            )*
        };
    }
macro_rules! impl_fast_math_for_float {
        ($($t:ty),*) => {
            $(
                impl UnsafeMath for $t {
                    #[inline(always)] fn fast_add(self, rhs: Self) -> Self { unsafe { core::intrinsics::fadd_fast(self, rhs) } }
                    #[inline(always)] fn fast_sub(self, rhs: Self) -> Self { unsafe { core::intrinsics::fsub_fast(self, rhs) } }
                    #[inline(always)] fn fast_mul(self, rhs: Self) -> Self { unsafe { core::intrinsics::fmul_fast(self, rhs) } }
                    #[inline(always)] fn fast_div(self, rhs: Self) -> Self { unsafe { core::intrinsics::fdiv_fast(self, rhs) } }
                    #[inline(always)] fn fast_rem(self, rhs: Self) -> Self { unsafe { core::intrinsics::frem_fast(self, rhs) } }
                    #[inline(always)] fn fast_shl(self, _rhs: u32) -> Self { unsafe { std::hint::unreachable_unchecked() } }
                    #[inline(always)] fn fast_shr(self, _rhs: u32) -> Self { unsafe { std::hint::unreachable_unchecked() } }
                }
            )*
        };
    }

impl_fast_math_for_int!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);
impl_fast_math_for_float!(f32, f64);
