use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sample::hash;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash 2", |b| {
        b.iter(|| {
            let n = black_box(2);
            hash(n)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
