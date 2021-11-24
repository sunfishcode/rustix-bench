use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

mod suite {
    #[inline(never)]
    pub(super) fn simple_open_std(path: &str) {
        let _ = std::fs::File::open(path).unwrap();
    }

    #[inline(never)]
    pub(super) fn simple_read_std(path: &str) {
        let _ = std::fs::read_to_string(path).unwrap();
    }

    #[inline(never)]
    pub(super) fn simple_stat_std(path: &str) {
        let _ = std::fs::metadata(path).unwrap();
    }

    #[inline(never)]
    pub(super) fn simple_clock_gettime_std() {
        let _ = std::time::Instant::now();
    }
}

fn bench_fs(c: &mut Criterion) {
    let mut group = c.benchmark_group("fs");
    group.bench_with_input(BenchmarkId::new("std", "stat"), &(), |b, _i| {
        b.iter(|| suite::simple_stat_std("/"))
    });
    group.bench_with_input(BenchmarkId::new("std", "open"), &(), |b, _i| {
        b.iter(|| suite::simple_open_std("/etc/services"))
    });
    group.bench_with_input(BenchmarkId::new("std", "read"), &(), |b, _i| {
        b.iter(|| suite::simple_read_std("/etc/services"))
    });
    group.finish();
}

fn bench_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("time");
    group.bench_with_input(BenchmarkId::new("std", "clock_gettime"), &(), |b, _i| {
        b.iter(|| suite::simple_clock_gettime_std())
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_fs,
    bench_time,
);

criterion_main!(benches);
