use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day6;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input").unwrap();
    c.bench_function("verq: 4 char", |b| {
        b.iter(|| day6::find_start(black_box(&input), black_box(4)))
    });

    c.bench_function("allison: 4 char", |b| {
        b.iter(|| day6::allison_find_n_unique(input.as_bytes(), black_box(4)))
    });
    // c.bench_function("kappa: 4 char", |b| {
    //     b.iter(|| day6::kappa(black_box(&input), black_box(4)))
    // });
    c.bench_function("verq: 14 char", |b| {
        b.iter(|| day6::find_start(black_box(&input), black_box(14)))
    });

    c.bench_function("allison: 14 char", |b| {
        b.iter(|| day6::allison_find_n_unique(input.as_bytes(), black_box(14)))
    });

    // c.bench_function("verq: 1400 char with 4K input", |b| {
    //     b.iter(|| day6::find_start(black_box(&"a".repeat(4098)), black_box(1400)))
    // });
    // c.bench_function("kappa: 14 char", |b| {
    //     b.iter(|| day6::kappa(black_box(&input), black_box(14)))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
