use criterion::*;

fn sandbox_benchmark(criterion: &mut Criterion) {
    // let mut sandbox = Sandbox::new(512, 512);

    // criterion.bench_function("", |b| )
}

criterion_group!(benches, sandbox_benchmark);
criterion_main!(benches);
