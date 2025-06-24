#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]
#![feature(test)]
extern crate test;

use unsafe_math::unsafe_math;

// no_mangle is for cargo asm

#[unsafe(no_mangle)]
fn sliding_sum_baseline(data: &[u32], window: usize) -> u64 {
    let mut out = 0;
    for i in 0..data.len() - window {
        let mut sum = 0;
        for j in 0..window {
            sum += data[i + j];
        }
        out += sum as u64;
    }
    out
}

#[unsafe(no_mangle)]
fn sliding_sum_wrapping(data: &[u32], window: usize) -> u64 {
    let mut out = 0u64;
    for i in 0..data.len() - window {
        let mut sum = 0u32;
        for j in 0..window {
            sum = sum.wrapping_add(data[i + j]);
        }
        out = out.wrapping_add(sum as u64);
    }
    out
}

#[unsafe(no_mangle)]
fn sliding_sum_fast(data: &[u32], window: usize) -> u64 {
    let mut out = 0;
    // you can use unsafe_math on for loops, too. But you need stmt_expr_attributes and proc_macro_hygiene features
    #[unsafe_math]
    for i in 0..data.len() - window {
        let mut sum = 0;
        for j in 0..window {
            sum += data[i + j];
        }
        out += sum as u64;
    }
    out
}

#[unsafe(no_mangle)]
pub fn bilinear_sample_baseline(a00: f64, a10: f64, a01: f64, a11: f64, fx: f64, fy: f64) -> f64 {
    let inv_fx = 1.0 - fx;
    let inv_fy = 1.0 - fy;

    let w00 = inv_fx * inv_fy;
    let w10 = fx * inv_fy;
    let w01 = inv_fx * fy;
    let w11 = fx * fy;

    let mut result = 0.0f64;
    result += a00 * w00;
    result += a10 * w10;
    result += a01 * w01;
    result += a11 * w11;

    result
}

#[unsafe_math]
#[unsafe(no_mangle)]
pub fn bilinear_sample_fast(a00: f64, a10: f64, a01: f64, a11: f64, fx: f64, fy: f64) -> f64 {
    let inv_fx = 1.0 - fx;
    let inv_fy = 1.0 - fy;

    let w00 = inv_fx * inv_fy;
    let w10 = fx * inv_fy;
    let w01 = inv_fx * fy;
    let w11 = fx * fy;

    let mut result = 0.0f64;
    result += a00 * w00;
    result += a10 * w10;
    result += a01 * w01;
    result += a11 * w11;

    result
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;
    use test::Bencher;

    const VECTOR_SIZE: usize = 1 << 14;

    #[bench]
    fn bench_sliding_sum_baseline(b: &mut Bencher) {
        let data: Vec<u32> = (0..VECTOR_SIZE as u32)
            .map(|x| black_box(x.wrapping_mul(7).wrapping_add(3)))
            .collect();
        let window = 128;

        (0..1000)
            .map(|_| black_box(sliding_sum_wrapping(&data, window)))
            .for_each(drop);

        b.iter(|| black_box(sliding_sum_baseline(&data, window)))
    }

    #[bench]
    fn bench_sliding_sum_wrapping(b: &mut Bencher) {
        let data: Vec<u32> = (0..VECTOR_SIZE as u32)
            .map(|x| black_box(x.wrapping_mul(7).wrapping_add(3)))
            .collect();
        let window = 128;

        (0..1000)
            .map(|_| black_box(sliding_sum_wrapping(&data, window)))
            .for_each(drop);

        b.iter(|| black_box(sliding_sum_wrapping(&data, window)))
    }

    #[bench]
    fn bench_sliding_sum_fast(b: &mut Bencher) {
        let data: Vec<u32> = (0..VECTOR_SIZE as u32)
            .map(|x| black_box(x.wrapping_mul(7).wrapping_add(3)))
            .collect();
        let window = 128;

        (0..1000)
            .map(|_| black_box(sliding_sum_wrapping(&data, window)))
            .for_each(drop);

        b.iter(|| black_box(sliding_sum_fast(&data, window)))
    }

    const A00: f64 = black_box(0.1f64);
    const A10: f64 = black_box(0.9f64);
    const A01: f64 = black_box(0.2f64);
    const A11: f64 = black_box(0.8f64);
    const FX: f64 = black_box(0.33f64);
    const FY: f64 = black_box(0.66f64);

    #[bench]
    fn bench_bilinear_sample_baseline(b: &mut Bencher) {
        b.iter(|| {
            black_box(bilinear_sample_baseline(
                black_box(A00),
                black_box(A10),
                black_box(A01),
                black_box(A11),
                black_box(FX),
                black_box(FY),
            ))
        })
    }

    #[bench]
    fn bench_bilinear_sample_fast(b: &mut Bencher) {
        b.iter(|| {
            black_box(bilinear_sample_fast(
                black_box(A00),
                black_box(A10),
                black_box(A01),
                black_box(A11),
                black_box(FX),
                black_box(FY),
            ))
        })
    }
}
