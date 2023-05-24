use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn import_sat_formula(file_name: String) -> Result<(Vec<Vec<i32>>, usize), String> {
    let mut num_vars: usize = 0;
    let mut clauses: Vec<Vec<i32>> = Vec::new();
    if let Ok(mut lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("c") {
                    continue;
                } else if ip.starts_with("p") {
                    let split = ip.split_whitespace().collect::<Vec<&str>>();
                    num_vars = split[2].parse::<usize>().unwrap();
                    clauses.reserve(split[3].parse::<usize>().unwrap());
                } else {
                    let clause = ip.trim().split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .filter(|x| *x != 0)
                    .collect::<Vec<i32>>();
                    if clause.len() > 0{
                        clauses.push(clause);
                    }
                }
            }
        }
    }

    return Result::Ok((clauses, num_vars));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
