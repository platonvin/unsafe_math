#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]
#![feature(test)]
extern crate test;

use unsafe_math::*;

// this "benchmark" exists mostly to compare assembly
// maybe i should use test for this

#[unsafe(no_mangle)]
fn slow_convert(block: i32) -> i32 {
    (block * 16) / 8
}

#[unsafe(no_mangle)]
#[unsafe_math]
fn fast_convert(block: i32) -> i32 {
    (block * 16) / 8
}

#[unsafe(no_mangle)]
fn slow_sum(a: u32) -> u32 {
    (0..a).map(|i| 1 << i as u32).sum()
}

#[unsafe(no_mangle)]
#[unsafe_math]
fn fast_sum(a: u32) -> u32 {
    (0..a).map(|i| 1 << i as u32).sum()
}

#[unsafe(no_mangle)]
fn smart_sum(a: u32) -> u32 {
    (2 << (a - 1)) - 1
}

#[cfg(test)]
mod tests {
    use unsafe_math_macro::unsafe_math_block;

    use crate::*;
    use std::hint::black_box;

    #[test]
    fn smart_is_correct() {
        let a = black_box(9);
        let slow = slow_sum(a);
        let fast = fast_sum(a);
        let smart = smart_sum(a);

        assert_eq!(slow, fast);
        assert_eq!(slow, smart);
    }

    const ITERS: u32 = 24;
    #[bench]
    fn bench_sum_slow(b: &mut test::Bencher) {
        b.iter(|| {
            (0..ITERS)
                .map(|i| black_box(slow_sum(black_box(i))))
                .for_each(drop)
        });
    }
    #[bench]
    fn bench_sum_fast(b: &mut test::Bencher) {
        b.iter(|| {
            (0..ITERS)
                .map(|i| black_box(fast_sum(black_box(i))))
                .for_each(drop)
        });
    }
    #[bench]
    fn bench_sum_smart(b: &mut test::Bencher) {
        b.iter(|| {
            (0..ITERS)
                .map(|i| black_box(smart_sum(black_box(i))))
                .for_each(drop) // of wisdom
        });
    }

    // should be equivalent to fast
    #[bench]
    fn bench_sum_fast_scope(b: &mut test::Bencher) {
        b.iter(|| {
            (0..ITERS)
                .map(|i| {
                    black_box(
                        #[unsafe_math]
                        {
                            let a = black_box(i);
                            (0..a).map(|i| 1 << i as u32).sum::<u32>()
                        },
                    )
                })
                .for_each(drop)
        });
    }

    // should be equivalent to fast
    #[bench]
    fn bench_sum_fast_statements(b: &mut test::Bencher) {
        b.iter(|| {
            (0..ITERS)
                .map(|i| {
                    black_box(
                        #[unsafe_math]
                        {
                            let a = black_box(i);
                            let mut sum = 0;
                            #[unsafe_math]
                            for i in 0..a {
                                {
                                    // you can nest them, it doesnt break anything
                                    unsafe_math_block! {
                                        sum += 1 << i
                                    }
                                }
                            }
                            sum
                        },
                    )
                })
                .for_each(drop)
        });
    }

    struct UnsafeCalc {}
    trait UnsafeCalcTrait {
        fn calc(sum: &mut i32, i: u32);
    }

    #[unsafe_math]
    impl UnsafeCalcTrait for UnsafeCalc {
        fn calc(sum: &mut i32, i: u32) {
            *sum += 1 << i;
        }
    }

    // should be equivalent to fast
    #[bench]
    fn bench_sum_fast_trait(b: &mut test::Bencher) {
        b.iter(|| {
            (0..ITERS)
                .map(|i| {
                    black_box({
                        let a = black_box(i);
                        let mut sum = 0;
                        for i in 0..a {
                            {
                                UnsafeCalc::calc(&mut sum, i);
                            }
                        }
                        sum
                    })
                })
                .for_each(drop)
        });
    }
}
