use core::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use data_oriented_example::{
    gen_vec_dyn, gen_vecs, gen_vecs_box, run_dyn, run_vecs, run_vecs_box, MyTrait,
};

fn bench_dyntrait(c: &mut Criterion) {
    let mut group = c.benchmark_group("DynTrait");
    group.warm_up_time(Duration::from_millis(1000));
    group.measurement_time(Duration::from_millis(15000));
    group.sample_size(10);

    for i in [
        100, 1000, 2000, 5000, 10000, 50000, 100000, 1000000, 3000000, 5000000,
    ]
    .iter()
    {
        let vec = gen_vecs(*i);
        let vec_box = gen_vecs_box(*i);
        let vec_dyn: Vec<Box<dyn MyTrait>> = gen_vec_dyn(*i);

        group.bench_with_input(BenchmarkId::new("Vec<T>", i), i, |b, i| {
            b.iter(|| {
                run_vecs(&vec.0);
                run_vecs(&vec.1);
            })
        });
        group.bench_with_input(BenchmarkId::new("Vec<Box<T>>", i), i, |b, i| {
            b.iter(|| {
                run_vecs_box(&vec_box.0);
                run_vecs_box(&vec_box.1);
            })
        });
        group.bench_with_input(BenchmarkId::new("Vec<dyn T>", i), i, |b, i| {
            b.iter(|| {
                run_dyn(&vec_dyn);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_dyntrait);
criterion_main!(benches);
