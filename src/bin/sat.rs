use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use sat::*;

fn main() -> Result<(), String> {
    match std::env::args().collect::<Vec<_>>().as_slice() {
        [_, path] => {
            let start = Instant::now();

            let lines = File::open(path)
                .map(|f| BufReader::new(f).lines().filter_map(Result::ok))
                .map_err(|e| format!("{}", e))?;

            let solution = Formula::parse_and_solve(lines)?;
            print_solution(solution);
            println!("c solved in {}ms", start.elapsed().as_millis());
            Ok(())
        }
        _ => Err("Usage: ./run.sh <problem file>".into()),
    }
}

fn print_solution(solution: Solution<impl IntoIterator<Item = (Variable, Sign)>>) {
    match solution {
        Solution::Unsat => println!("s UNSATISFIABLE"),
        Solution::Sat(assignment) => {
            println!("s SATISFIABLE");
            print!("v");
            for (var, sign) in assignment {
                print!(" {}{}", sign, var);
            }
            println!(" 0");
        }
    }
}
