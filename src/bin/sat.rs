use std::time::Instant;

use sat::*;

fn main() -> Result<(), String> {
    match std::env::args().collect::<Vec<_>>().as_slice() {
        [_, path] => {
            let start = Instant::now();
            let solution = Formula::parse_and_solve_file(path)?;
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
