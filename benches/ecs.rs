use criterion::*;
use shard_ecs::*;
use rand::{*, seq::SliceRandom};

const COUNT: usize = 1_000_000;

fn criterion_benchmark(c: &mut Criterion) {
    let mut registry = Registry::default();
    
    let entities = (0..COUNT).into_iter().map(|_|{
        let p = P {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        };
        let r = R {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        };
        let s = S {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        };

        if rand::random() {
            registry.create_entity((p, s)).unwrap()
        } else if rand::random() {
            registry.create_entity(p).unwrap()
        } else {
            registry.create_entity((p, r, s)).unwrap()
        }
    }).collect::<Vec<_>>();
    
    c.bench_function("random_lookup", |b|{
        b.iter_batched(||{
            entities.choose(&mut thread_rng()).unwrap().clone()
        }, |entity| {
            registry.get_component::<P>(black_box(entity));
        }, BatchSize::LargeInput)
    });

}

criterion_group!(ecs, criterion_benchmark);
criterion_main!(ecs);


#[derive(Debug)]
struct P {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Debug)]
struct R {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(Debug)]
struct S {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for P {
    const NAME: &'static str = "P";
}
impl Component for R {
    const NAME: &'static str = "R";
}
impl Component for S {
    const NAME: &'static str = "S";
}
