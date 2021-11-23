use criterion::{criterion_group, criterion_main, Criterion};

use aoc_2020::day01::*;

// TODO: impl with inputs?
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    //     {
    //         let mut pair_impls = c.benchmark_group("Pair");

    //         pair_impls.bench_function("naive", |b| {
    //             b.iter(|| find_pair_with_sum_naive(&INPUTS, TARGET_SUM))
    //         });
    //         pair_impls.bench_function("itertools", |b| {
    //             b.iter(|| find_pair_with_sum(&INPUTS, TARGET_SUM))
    //         });
    //     }

    //     c.bench_function("triple", |b| {
    //         b.iter(|| find_triple_with_sum(&INPUTS, TARGET_SUM))
    //     });
}
