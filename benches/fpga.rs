use criterion::{criterion_group, criterion_main, Criterion};
use sat::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn fpga(c: &mut Criterion) {
    let lines = BufReader::new(File::open("inputs/fpga.cnf").unwrap())
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    c.bench_function("fpga (sat) solve", |b| {
        b.iter(|| Formula::parse_and_solve(&lines).unwrap())
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(50);
    targets = fpga
}
criterion_main!(benches);
