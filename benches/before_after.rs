use broken_app::{algo, algo_broken};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

fn bench_fib_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("fib_comparison");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("before_slow_fib_20", |b| b.iter(|| algo_broken::slow_fib(20)));
    group.bench_function("before_slow_fib_25", |b| b.iter(|| algo_broken::slow_fib(25)));
    group.bench_function("after_fast_fib_20", |b| b.iter(|| algo::fast_fib(20)));
    group.bench_function("after_fast_fib_25", |b| b.iter(|| algo::fast_fib(25)));
    group.bench_function("after_fast_fib_32", |b| b.iter(|| algo::fast_fib(32)));
    group.finish();
}

fn bench_dedup_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("dedup_comparison");

    for size in [100, 500, 1000, 2000].iter() {
        let data: Vec<u64> = (0..*size).flat_map(|n| [n, n]).collect();

        group.bench_with_input(
            BenchmarkId::new("before_slow_dedup", size),
            &data,
            |b, data| b.iter(|| algo_broken::slow_dedup(data)),
        );
        group.bench_with_input(
            BenchmarkId::new("after_optimized_dedup", size),
            &data,
            |b, data| b.iter(|| algo::optimized_dedup(data)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_fib_comparison, bench_dedup_comparison);
criterion_main!(benches);