use criterion::{criterion_group, criterion_main, Criterion};
use sat::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn bf0432(c: &mut Criterion) {
    let lines = BufReader::new(File::open("inputs/bf0432-007.cnf").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    c.bench_function("bf0432-007 solve", |b| {
        b.iter(|| Solver::parse_and_solve(&lines).unwrap())
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bf0432
}
criterion_main!(benches);
