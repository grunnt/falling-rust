use criterion::*;
use falling_rust::element::Element;
use falling_rust::sandbox::SandBox;
use falling_rust::simulation::{simulation_step, Simulation};

// Note: to get a meaningful benchmark avoid simulations that become static afer a number of iterations
fn criterion_benchmark(criterion: &mut Criterion) {
    let size = 64;
    let mut simulation = Simulation::default();

    // Empty sandbox (should be fast)
    let mut sandbox = SandBox::new(size, size, Some(1234));
    criterion.bench_function("empty_simulation", |b| {
        b.iter(|| simulation_step(&mut simulation, &mut sandbox))
    });

    // Water flowing from top to bottom (pretty much slowest element)
    let mut sandbox = SandBox::new(size, size, Some(1234));
    for x in 0..size / 4 {
        sandbox.set_element(x * 4, 1, Element::WaterSource);
    }
    for x in 0..size / 3 {
        sandbox.set_element(x * 3, size - 1, Element::Drain);
    } 
    criterion.bench_function("water_flow_simulation", |b| {
        b.iter(|| simulation_step(&mut simulation, &mut sandbox))
    });

    // Oil and fire
    let mut sandbox = SandBox::new(size, size, Some(1234));
    for x in 0..size / 4 {
        sandbox.set_element(x * 4, 1, Element::OilSource);
    }
    for x in 0..size / 3 {
        sandbox.set_element(x * 3, size - 1, Element::FireSource);
    }
    criterion.bench_function("burning_oil_simulation", |b| {
        b.iter(|| simulation_step(&mut simulation, &mut sandbox))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
