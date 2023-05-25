mod input_output;
mod solver;

use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

extern crate num_cpus;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (clauses, num_vars) =
        input_output::read_write_file::import_sat_formula(args[1].clone()).unwrap();
    let out_file = args[2].clone();
    let num_threads = if args.len() > 3 {
        args[3].parse::<usize>().unwrap()
    } else {
        num_cpus::get()
    };

    println!("number of threads: {}", num_threads);

    let halt = Arc::new(Mutex::new(false));
    let mut handles = vec![];
    for _ in 0..num_threads {
        let num_vars_clone = num_vars.clone();
        let clauses_clone = clauses.clone();
        let halt_clone = Arc::clone(&halt);
        handles.push(thread::spawn(move || {
            let mut a = solver::solver_manager::SolverManager::new(
                num_vars_clone,
                clauses_clone,
                halt_clone,
            );
            return a.solve();
        }));
    }

    let mut check_handle: usize = 0;
    loop {
        if handles[check_handle].is_finished() {
            break;
        }
        check_handle += 1;
        check_handle %= num_threads;
    }
    let assignment: Vec<bool> = handles.swap_remove(check_handle).join().unwrap();

    let mut out = "s SATISFIABLE\nv ".to_string();
    for i in 1..assignment.len() {
        if assignment[i] == true {
            out += i.to_string().as_str();
        } else {
            out += (-(i as i32)).to_string().as_str();
        }

        out += " "
    }
    out += "0";
    fs::write(out_file, out).unwrap();
}
