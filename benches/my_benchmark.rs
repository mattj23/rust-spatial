#[macro_use]
extern crate criterion;

extern crate spatial;

use criterion::Criterion;
use spatial::vec3::Vec3;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Vector Add", |b| b.iter(|| Vec3::new(1., 2., 3.) + Vec3::new(4., 5., 6.)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
