# SAT
![Build](https://github.com/mxheller/sat/workflows/Rust/badge.svg)

A CDCL SAT solver written in Rust

## Say What?!

A [**C**onflict-**D**riven **C**lause **L**earning](https://en.wikipedia.org/wiki/Conflict-driven_clause_learning) [**Sat**isfiability](https://en.wikipedia.org/wiki/Boolean_satisfiability_problem) solver.
Essentially, a program for determining whether there exists an assignment of variables such that a boolean formula such as
```
(a ∨ b) ∧ (¬a ∨ c)
```
evaluates to true, where `∨` means OR, `∧` means AND, `¬` means NOT, and *assignment* means something of the form
```
a: false
b: true
c: true
```

## Installation

After [installing rust](https://www.rust-lang.org/tools/install) and cloning the repo, you can install `sat` by running

```
cargo install --path .
```

Note the installation path `cargo` outputs at the end, e.g. `/home/mxheller/.cargo/bin/sat`, and make sure that its containing folder (`/home/mxheller/.cargo/bin` in this case) is in your `PATH`.

## Usage

After installation, you can run `sat` on a problem file using
```
sat <problem file>
```

Alternatively, you can run `sat` without installing it by executing
```
cargo run --release sat <file>
```

### Input/Output Format

`sat` accepts input and produces output in the SAT Competition format ([see sections 4.1 and 5.2](http://www.satcompetition.org/2011/rules.pdf)).

Given the following satisfiable input
```
p cnf 3 2
1 -2 3 0
-3     0
```
`sat` might produce
```
s SATISFIABLE
v 1 -2 -3 0
c solved in 0ms
```

Given the following unsatisfiable input
```
p cnf 3 2
1 -2 3 0
-3     0
```
`sat` (should hopefully!) produce
```
s UNSATISFIABLE
c solved in 0ms
```
