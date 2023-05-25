use super::sls_solver::{self, Solver};
use std::sync::{Mutex, Arc};

pub struct SolverManager {
    pub solver: sls_solver::Solver,
    /// upon creation, takes a pointer to a bool that will be used to communicate with the other threads
    pub clauses : Vec<Vec<i32>>,
    pub num_vars: usize,
    halt : Arc<Mutex<bool>>
}

impl SolverManager{
    pub fn new(num_vars: usize, clauses: Vec<Vec<i32>>, halt : Arc<Mutex<bool>>) -> SolverManager{
        let solver = sls_solver::Solver::new(num_vars, clauses.clone());

        return SolverManager{
            solver,
            clauses,
            num_vars,
            halt
        }
    }

    pub fn solve(self: &mut SolverManager) -> Vec<bool>{
        
        loop {
            {
                let halt = self.halt.lock().unwrap();
                if *halt {
                    return vec![];
                }
            }
            self.solver.solve();
            if self.solver.is_solved() {
                let mut halt = self.halt.lock().unwrap();
                *halt = true;
                return self.solver.assignment.clone();
            }
            self.solver = sls_solver::Solver::new(self.num_vars, self.clauses.clone());
        }
    }
}
