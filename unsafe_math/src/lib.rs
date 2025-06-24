//! # unsafe_math main crate
//!
//! Re-exports unsafe_math macro and trait (proc_macro crates cant export traits)
//!
//! See the project [README](https://github.com/your-username/unsafe_math/blob/main/README.md) for details

pub use unsafe_math_macro::unsafe_math;
pub use unsafe_math_trait::UnsafeMath;

// TODO: i am actually unsure how should i test it
// i dont want macro to break anything, but UB does not have to equal wrapping_add / sub, does it?

#[cfg(test)]
mod tests {
    use std::ops::{Add, Div, Mul, Rem, Sub};

    use unsafe_math_macro::unsafe_math;
    use unsafe_math_trait::UnsafeMath;

    #[test]
    fn test_integer_fast_add() {
        let a: u32 = 1;
        let b: u32 = 2;
        assert_eq!(unsafe { a.unchecked_add(b) }, a.wrapping_add(b));
        assert_eq!(a.fast_add(b), unsafe { a.unchecked_add(b) });
    }

    #[test]
    fn test_integer_overflow_behaviour() {
        let max: u8 = u8::MAX;
        let one: u8 = 1;
        let wrapped = max.wrapping_add(one);
        let fast = max.fast_add(one);
        assert_eq!(fast, wrapped);
    }

    #[test]
    fn test_integer_other_ops() {
        let x: i16 = -5;
        let y: i16 = 3;
        // i use checked but its not like there is a reason, i just like the contrast
        assert_eq!(x.fast_sub(y), x.checked_sub(y).unwrap());
        assert_eq!(x.fast_mul(y), x.checked_mul(y).unwrap());
        assert_eq!(x.fast_div(y), x.checked_div(y).unwrap());
        assert_eq!(x.fast_rem(y), x.checked_rem(y).unwrap());
        assert_eq!(x.fast_shl(2), x.checked_shl(2).unwrap());
        assert_eq!(x.fast_shr(1), x.checked_shr(1).unwrap());
    }

    #[test]
    fn test_float_fast_math() {
        let f: f32 = 1.5;
        let g: f32 = 2.25;
        assert_eq!(f.fast_add(g), f.add(g));
        assert_eq!(f.fast_sub(g), f.sub(g));
        assert_eq!(f.fast_mul(g), f.mul(g));
        assert_eq!(f.fast_div(g), f.div(g));
        assert_eq!(f.fast_rem(g), f.rem(g));
    }

    #[unsafe_math]
    fn calc_int(a: u32, b: u32) -> u32 {
        a * b + a - b
    }

    #[test]
    fn test_calc_int_agrees() {
        for i in 0..50 {
            for j in 0..50 {
                assert_eq!(
                    calc_int(i, j),
                    i.wrapping_mul(j).wrapping_add(i).wrapping_sub(j)
                );
            }
        }
    }

    #[unsafe_math]
    fn sum_of_squares(n: u16) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..=n {
            sum += i as u32 * i as u32;
        }
        sum
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(0), 0);
        assert_eq!(sum_of_squares(1), 1);
        assert_eq!(
            sum_of_squares(10),
            (0..=10).map(|i| (i as u32) * (i as u32)).sum()
        );
    }

    #[unsafe_math]
    fn calc_float(x: f32, y: f32) -> f32 {
        (x * x + y * y).sqrt()
    }

    #[test]
    fn test_calc_float_agrees() {
        for &x in &[0.0f32, 1.5, -2.3, 3.14] {
            for &y in &[0.0f32, 2.0, -4.5, 6.28] {
                let expected = (x * x + y * y).sqrt();
                let result = calc_float(x, y);
                // allow small epsilon for floating differences
                let diff = (expected - result).abs();
                // println!("diff={diff}");
                assert!(diff < 1e-6);
            }
        }
    }
}
