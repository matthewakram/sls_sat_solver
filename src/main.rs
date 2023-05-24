mod input_output;
mod solver;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (clauses, num_vars) = input_output::read_write_file::import_sat_formula(args[1].clone()).unwrap();

    
    let solver = solver::sls_solver::Solver::new(num_vars, clauses);
    println!("{:?}", solver.clauses);
    println!("{:?}", solver.assignment);
    println!("{:?}", solver.current_clause_assignment);
    println!("{}", solver.cost.peek().unwrap());
}
