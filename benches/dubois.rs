use criterion::{criterion_group, criterion_main, Criterion};
use sat::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn dubois(c: &mut Criterion) {
    let lines = BufReader::new(File::open("inputs/dubois.cnf").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    c.bench_function("dubois solve", |b| {
        b.iter(|| Solver::parse_and_solve(&lines).unwrap())
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = dubois
}
criterion_main!(benches);
