#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

//! # unsafe_math main crate
//!
//! Re-exports unsafe_math macro and trait (proc_macro crates cant export traits)
//!
//! See the project [README](https://github.com/platonvin/unsafe_math/blob/master/README.md) for details

pub use unsafe_math_macro::unsafe_math;
pub use unsafe_math_macro::unsafe_math_block;
pub use unsafe_math_trait::UnsafeMath;

// TODO: i am actually unsure how should i test it
// i dont want macro to break anything, but UB does not have to equal wrapping_add / sub, does it?

// this only tests validity, not if unsafe_math actually does anything
#[cfg(test)]
mod tests {
    use std::ops::{Add, Div, Mul, Rem, Sub};

    use unsafe_math_macro::unsafe_math_block;

    use crate::unsafe_math;

    #[test]
    fn test_integer_fast_add() {
        use unsafe_math_trait::UnsafeMath;

        let a: u32 = 1;
        let b: u32 = 2;
        assert_eq!(unsafe { a.unchecked_add(b) }, a.wrapping_add(b));
        assert_eq!(a.fast_add(b), unsafe { a.unchecked_add(b) });
    }

    #[test]
    fn test_integer_overflow_behaviour() {
        use unsafe_math_trait::UnsafeMath;

        let max: u8 = u8::MAX;
        let one: u8 = 1;
        let wrapped = max.wrapping_add(one);
        let fast = max.fast_add(one);
        assert_eq!(fast, wrapped);
    }

    #[test]
    fn test_integer_other_ops() {
        use unsafe_math_trait::UnsafeMath;

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
        use unsafe_math_trait::UnsafeMath;

        let f: f32 = 1.5;
        let g: f32 = 2.25;
        assert_eq!(f.fast_add(g), f.add(g));
        assert_eq!(f.fast_sub(g), f.sub(g));
        assert_eq!(f.fast_mul(g), f.mul(g));
        assert_eq!(f.fast_div(g), f.div(g));
        assert_eq!(f.fast_rem(g), f.rem(g));
    }

    fn calc_int(a: u32, b: u32) -> u32 {
        #[unsafe_math]
        {
            a * b + a - b
        }
    }

    #[test]
    fn test_calc_int_agrees() {
        for i in 0..50_u32 {
            for j in 0..50_u32 {
                assert_eq!(
                    calc_int(i, j),
                    i.wrapping_mul(j).wrapping_add(i).wrapping_sub(j)
                );
            }
        }
    }

    fn sum_of_squares_ub_wraps(n: u16) -> u16 {
        let mut sum: u16 = 0;
        #[unsafe_math]
        for i in 0..=n {
            sum += i as u16 * i as u16;
        }
        sum as u16
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares_ub_wraps(0), 0);
        assert_eq!(sum_of_squares_ub_wraps(1), 1);
        assert_eq!(
            sum_of_squares_ub_wraps(420),
            (0..=420)
                .map(|i| (i as u16).wrapping_mul(i as u16))
                .fold(0, |a: u16, b: u16| a.wrapping_add(b))
        );
    }

    #[unsafe_math]
    fn calc_float(x: f32, y: f32) -> f32 {
        #[unsafe_math]
        let x2 = x * x;
        #[unsafe_math]
        let y2 = y * y;
        #[unsafe_math]
        let sum = (x2 + y2);

        sum.sqrt()
    }

    macro_rules! assert_float_eq {
        ($a:expr, $b:expr, $epsilon:expr) => {
            assert!(
                ($a - $b).abs() < $epsilon,
                "Floats not equal: {} vs {}",
                $a,
                $b
            )
        };
    }

    #[test]
    fn test_calc_float_agrees() {
        for &x in &[0.0f32, 1.5, -2.3, 3.14] {
            for &y in &[0.0f32, 2.0, -4.5, 6.28] {
                let expected = (x * x + y * y).sqrt();
                let result = calc_float(x, y);

                assert_float_eq!(expected, result, 1e-6)
            }
        }
    }

    // parenthesis tests. Im scared of changing order of operations

    #[unsafe_math]
    fn deeply_nested_add(a: u32, b: u32, c: u32) -> u32 {
        ((a + b) + c)
    }

    #[test]
    fn test_deeply_nested_add() {
        let a = 10_u32;
        let b = 20_u32;
        let c = 30_u32;
        let expected = a + b + c;
        assert_eq!(deeply_nested_add(a, b, c), expected);
    }

    #[unsafe_math]
    fn precedence_int_mul_add(a: u32, b: u32, c: u32) -> u32 {
        a + (b * c)
    }

    #[unsafe_math]
    fn precedence_int_add_mul(a: u32, b: u32, c: u32) -> u32 {
        (a + b) * c
    }

    #[test]
    fn test_precedence_int() {
        let a = 5_u32;
        let b = 10_u32;
        let c = 2_u32;

        let expected_mul_add = a + (b * c);
        assert_eq!(precedence_int_mul_add(a, b, c), expected_mul_add);

        // (a + b) * c
        let expected_add_mul = (a + b) * c;
        assert_eq!(precedence_int_add_mul(a, b, c), expected_add_mul);
    }

    #[unsafe_math]
    fn precedence_float_mul_add(a: f32, b: f32, c: f32) -> f32 {
        a + (b * c)
    }

    #[unsafe_math]
    fn precedence_float_add_mul(a: f32, b: f32, c: f32) -> f32 {
        (a + b) * c
    }

    #[test]
    fn test_precedence_float() {
        let a = 5.5f32;
        let b = 10.0f32;
        let c = 2.0f32;
        let epsilon = 1e-6f32;

        let expected_mul_add = a + (b * c);
        assert_float_eq!(precedence_float_mul_add(a, b, c), expected_mul_add, epsilon);

        let expected_add_mul = (a + b) * c;
        assert_float_eq!(precedence_float_add_mul(a, b, c), expected_add_mul, epsilon);
    }

    #[unsafe_math]
    fn chained_ops_int(a: i32, b: i32, c: i32, d: i32) -> i32 {
        ((a + b) * c) / d
    }

    #[test]
    fn test_chained_ops_int() {
        let a = 10_i32;
        let b = 5_i32;
        let c = 2_i32;
        let d = 3_i32;
        let expected = ((a + b) * c) / d;
        assert_eq!(chained_ops_int(a, b, c, d), expected);

        let a_ov = i32::MAX - 10;
        let b_ov = 20;
        let c_ov = 2;
        let d_ov = 3;
        let expected_ov = a_ov
            .wrapping_add(b_ov)
            .wrapping_mul(c_ov)
            .wrapping_div(d_ov);
        assert_eq!(chained_ops_int(a_ov, b_ov, c_ov, d_ov), expected_ov);
    }

    #[unsafe_math]
    fn chained_ops_float(a: f32, b: f32, c: f32, d: f32) -> f32 {
        ((a + b) * c) / d
    }

    #[test]
    fn test_chained_ops_float() {
        let a = 10.5f32;
        let b = 5.2f32;
        let c = 2.1f32;
        let d = 3.0f32;
        let epsilon = 1e-6f32;

        let expected = ((a + b) * c) / d;
        assert_float_eq!(chained_ops_float(a, b, c, d), expected, epsilon);
    }

    fn single_operand_parentheses(a: u32, b: u32) -> u32 {
        #[unsafe_math]
        {
            (a) + (b)
        }
    }

    #[test]
    fn test_single_operand_parentheses() {
        let a = 100_u32;
        let b = 200_u32;
        let expected = a + b;
        assert_eq!(single_operand_parentheses(a, b), expected);
    }

    fn mock_function() -> u32 {
        42
    }

    #[unsafe_math]
    fn parentheses_around_function_call(x: u32) -> u32 {
        (mock_function() * x) + 5
    }

    #[test]
    fn test_parentheses_around_function_call() {
        let x = 2;
        let expected = (mock_function() * x) + 5;
        assert_eq!(parentheses_around_function_call(x), expected);
    }

    fn mock_float_function() -> f32 {
        3.14
    }

    #[unsafe_math]
    fn parentheses_around_float_function_call(x: f32) -> f32 {
        (mock_float_function() / x) - 1.0
    }

    #[test]
    fn test_parentheses_around_float_function_call() {
        let x = 2.0f32;
        let epsilon = 1e-6f32;
        let expected = (mock_float_function() / x) - 1.0;
        assert_float_eq!(parentheses_around_float_function_call(x), expected, epsilon);
    }

    fn mixed_nested_and_precedence(a: i32, b: i32, c: i32, d: i32) -> i32 {
        unsafe_math_block! {((a + (b * c)) - d)}
    }

    #[test]
    fn test_mixed_nested_and_precedence() {
        let a = 10_i32;
        let b = 3_i32;
        let c = 5_i32;
        let d = 2_i32;
        let expected = (a + (b * c)) - d;
        assert_eq!(mixed_nested_and_precedence(a, b, c, d), expected);

        let a_neg = -10_i32;
        let b_neg = -3_i32;
        let c_neg = 5_i32;
        let d_neg = -2_i32;
        let expected_neg = a_neg
            .wrapping_add(b_neg.wrapping_mul(c_neg))
            .wrapping_sub(d_neg);
        assert_eq!(
            mixed_nested_and_precedence(a_neg, b_neg, c_neg, d_neg),
            expected_neg
        );
    }
}
