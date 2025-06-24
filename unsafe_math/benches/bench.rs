use criterion::{BenchmarkId, Criterion, criterion_group};
use std::hint::black_box;
use unsafe_math::*;

// no_mangle is for cargo asm
const VECTOR_SIZE: usize = 16_384;

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

#[unsafe_math]
#[unsafe(no_mangle)]
fn sliding_sum_fast(data: &[u32], window: usize) -> u64 {
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

fn int_workloads(c: &mut Criterion) {
    let mut group = c.benchmark_group("integer workloads");

    let data: Vec<u32> = (0..VECTOR_SIZE as u32)
        .map(|x| black_box(x.wrapping_mul(7).wrapping_add(3)))
        .collect();
    let window = black_box(128);

    group.bench_function(BenchmarkId::new("sliding_sum", "baseline"), |b| {
        b.iter(|| black_box(sliding_sum_baseline(&data, window)))
    });
    group.bench_function(BenchmarkId::new("sliding_sum", "wrapping"), |b| {
        b.iter(|| black_box(sliding_sum_wrapping(&data, window)))
    });
    group.bench_function(BenchmarkId::new("sliding_sum", "fast"), |b| {
        b.iter(|| black_box(sliding_sum_fast(&data, window)))
    });

    group.finish();
}

pub fn float_workloads(c: &mut Criterion) {
    let mut group = c.benchmark_group("float workloads");

    // 2D inputs
    let a00 = black_box(0.1f64);
    let a10 = black_box(0.9f64);
    let a01 = black_box(0.2f64);
    let a11 = black_box(0.8f64);
    let fx = black_box(0.33f64);
    let fy = black_box(0.66f64);

    group.bench_function(BenchmarkId::new("bilinear_sample", "baseline"), |b| {
        b.iter(|| {
            black_box(bilinear_sample_baseline(
                black_box(a00),
                black_box(a10),
                black_box(a01),
                black_box(a11),
                black_box(fx),
                black_box(fy),
            ))
        })
    });
    group.bench_function(BenchmarkId::new("bilinear_sample", "fast"), |b| {
        b.iter(|| {
            black_box(bilinear_sample_fast(
                black_box(a00),
                black_box(a10),
                black_box(a01),
                black_box(a11),
                black_box(fx),
                black_box(fy),
            ))
        })
    });

    group.finish();
}

criterion_group!(benches, int_workloads, float_workloads);
fn main() {
    benches();
    criterion::Criterion::default()
        .sample_size(200)
        .warm_up_time(std::time::Duration::from_secs(3))
        .measurement_time(std::time::Duration::from_secs(5))
        .configure_from_args()
        .final_summary();
}
