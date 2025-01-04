use aoc_2024::problems::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1p1", |b| {
        b.iter(|| day1p1::solution(black_box("./input/day1.txt")))
    });
    c.bench_function("day1p2", |b| {
        b.iter(|| day1p2::solution(black_box("./input/day1.txt")))
    });
    c.bench_function("day2p1", |b| {
        b.iter(|| day2p1::solution(black_box("./input/day2.txt")))
    });
    c.bench_function("day2p2", |b| {
        b.iter(|| day2p2::solution(black_box("./input/day2.txt")))
    });
    c.bench_function("day3p1", |b| {
        b.iter(|| day3p1::solution(black_box("./input/day3.txt")))
    });
    // c.bench_function("day3p2", |b| {
    //     b.iter(|| day3p2::solution(black_box("./input/day3.txt")))
    // });
    c.bench_function("day4p1", |b| {
        b.iter(|| day4p1::solution(black_box("./input/day4.txt")))
    });
    // c.bench_function("day4p2", |b| {
    //     b.iter(|| day4p2::solution(black_box("./input/day4.txt")))
    // });
    c.bench_function("day5p1", |b| {
        b.iter(|| day5p1::solution(black_box("./input/day5.txt")))
    });
    c.bench_function("day5p2", |b| {
        b.iter(|| day5p2::solution(black_box("./input/day5.txt")))
    });
    c.bench_function("day6p1", |b| {
        b.iter(|| day6p1::solution(black_box("./input/day6.txt")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
