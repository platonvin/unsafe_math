#![feature(test)]
extern crate test;

use unsafe_math::*;

// this "benchmark" is fake and exists purely to compare assembly
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
}
