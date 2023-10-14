use std::time::Instant;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_compare::{naive::Naive, Contains};

const BAD_BOTS: &str = include_str!("../bad_bots.txt");
const USER_AGENTS: &str = include_str!("../user_agents.txt");

pub fn criterion_benchmark(c: &mut Criterion) {
    let bad_bots = BAD_BOTS
        .split('\n')
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let user_agents = USER_AGENTS.split('\n').collect::<Vec<_>>();

    let naive = Naive::new(bad_bots);

    c.bench_function("naive", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for i in 0..iters {
                let user_agent = user_agents[i as usize % user_agents.len()];
                black_box(naive.contains(user_agent));
            }
            start.elapsed()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
