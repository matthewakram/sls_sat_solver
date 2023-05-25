use std::collections::HashSet;
use rand::Rng;

pub struct Solver {
    pub num_vars: usize,
    pub clauses: Vec<Vec<i32>>,
    pub clauses_containing_variable: Vec<Vec<i32>>,
    pub assignment: Vec<bool>,
    pub current_clause_assignment: Vec<bool>,
    /// states what each clause evaluates to with the current assignment
    pub num_fulfilled_clauses: usize,
    pub cost: Vec<i32>,
}

impl Solver {
    pub fn new(num_vars: usize, mut clauses: Vec<Vec<i32>>) -> Self {
        let num_vars = num_vars + 1;
        let mut variable_in_clauses: Vec<Vec<i32>> = vec![Vec::new(); num_vars];
        let mut new_clauses: Vec<Vec<i32>> = vec![Vec::new()];
        new_clauses.append(&mut clauses);
        for i in 0..new_clauses.len() {
            for j in 0..new_clauses[i].len() {
                variable_in_clauses[abs(new_clauses[i][j])].push(if new_clauses[i][j] > 0 {
                    i as i32
                } else {
                    -(i as i32)
                });
            }
        }

        // we start with a random assignment of the variables
        let mut assignment: Vec<bool> = vec![];
        for _ in 0..num_vars {
            assignment.push(rand::thread_rng().gen_bool(0.5))
        }

        let mut clause_assignment = vec![];
        clause_assignment.reserve(new_clauses.len());
        let mut num_fulfilled_clauses = 0;
        for i in 0..new_clauses.len() {
            // evaluate the clause with no flipped variables
            let eval = evaluate_clause_with_flipped(&new_clauses, &assignment, i as i32, 0);
            num_fulfilled_clauses += if eval { 1 } else { 0 };
            clause_assignment.push(eval);
        }
        let mut cost: Vec<i32> = vec![];
        for i in 0..num_vars {
            cost.push(calculate_variable_cost(
                &new_clauses,
                &assignment,
                &clause_assignment,
                &variable_in_clauses[i],
                i,
            ));
        }

        assert_eq!(num_vars, variable_in_clauses.len());
        assert_eq!(num_vars, assignment.len());
        assert_eq!(new_clauses.len(), clause_assignment.len());

        Solver {
            num_vars,
            clauses: new_clauses,
            clauses_containing_variable: variable_in_clauses,
            assignment,
            current_clause_assignment: clause_assignment,
            num_fulfilled_clauses,
            cost,
        }
    }

    pub fn flip_var(self: &mut Solver, var_num: usize) {
        let mut neighbours: HashSet<usize> = HashSet::new();
        // update the assingment of all of the clauses
        for i in &self.clauses_containing_variable[var_num] {
            let evaluates_to =
                evaluate_clause_with_flipped(&self.clauses, &self.assignment, *i, var_num);

            if evaluates_to == true && self.current_clause_assignment[abs(*i)] == false {
                self.num_fulfilled_clauses += 1;
            } else if evaluates_to == false && self.current_clause_assignment[abs(*i)] == true {
                self.num_fulfilled_clauses -= 1;
            }

            self.current_clause_assignment[abs(*i)] = evaluates_to;

            for neighbour in &self.clauses[abs(*i)] {
                neighbours.insert(abs(*neighbour));
            }
        }

        // flip the variable
        self.assignment[var_num] = !self.assignment[var_num];

        // update the costs of all of its neighbours
        neighbours.insert(var_num);

        for neighbour in neighbours {
            // update the costs, but the cost might increase, so that is no good!!
            self.cost[neighbour] = calculate_variable_cost(
                &self.clauses,
                &self.assignment,
                &self.current_clause_assignment,
                &self.clauses_containing_variable[neighbour],
                neighbour,
            );
        }
    }

    pub fn flip_a_good_var(self: &mut Solver) -> bool {
        let min = get_first_negative(&self.cost);
        if min == 0 {
            return false;
        }
        self.flip_var(min as usize);
        return  true;
    }

    pub fn flip_random(self: &mut Solver){
        let random_index = rand::thread_rng().gen_range(1..self.num_vars);
        self.flip_var(random_index);
    }

    pub fn solve(self: &mut Solver) {
        let mut num_iters = 0;
        let mut num_retries:usize = 0;
        while !self.is_solved() {
            let flipped_a_good_var = self.flip_a_good_var();
            if !flipped_a_good_var {
                num_retries += 1;
                for _ in 0..num_retries%10{
                    self.flip_random();
                }
                if num_retries > self.num_vars*self.clauses.len(){
                    return;
                }
            }
            num_iters += 1;
        }
        println!("number of iterations required for solving {}", num_iters);
    }

    pub fn is_solved(self: &Solver) -> bool {
        return self.num_fulfilled_clauses == self.clauses.len();
    }
}

const fn abs(number: i32) -> usize {
    return if number < 0 {
        -number as usize
    } else {
        number as usize
    };
}

/// Calculate how many clauses become true when flipping this variable
fn calculate_variable_cost(
    clauses: &Vec<Vec<i32>>,
    assignment: &Vec<bool>,
    clause_assignment: &Vec<bool>,
    clauses_to_consider: &Vec<i32>,
    variable_num: usize,
) -> i32 {
    let mut cost: i32 = 0;
    for clause in clauses_to_consider {
        let clause_evaluates_to =
            evaluate_clause_with_flipped(clauses, assignment, *clause, variable_num);
        if clause_evaluates_to == false {
            cost += 1;
        } else {
            if clause_assignment[abs(*clause)] == false {
                cost -= 1;
            }
        }
    }
    return cost;
}

/// evaluate the given clause with the given variable flipped
fn evaluate_clause_with_flipped(
    clauses: &Vec<Vec<i32>>,
    assignment: &Vec<bool>,
    clause_num: i32,
    flipped_var: usize,
) -> bool {
    if clause_num < 0 && flipped_var != 0 && assignment[flipped_var] == true {
        return true;
    } else if clause_num > 0 && flipped_var != 0 && assignment[flipped_var] == false {
        return true;
    }
    let mut result = true;
    for var in &clauses[abs(clause_num)] {
        let var_name = abs(*var);
        if var_name == flipped_var {
            if if *var > 0 {
                !assignment[var_name]
            } else {
                assignment[var_name]
            } {
                return true;
            }
        } else {
            if if *var > 0 {
                assignment[var_name]
            } else {
                !assignment[var_name]
            } {
                return true;
            }
        }
        result = false
    }
    // The reason we do this, is now the empty clause returns true. This is theoretically not correct. But since our sls solver doesnt change the formula, this just helps us avoid errors with the first empty clause
    // that is inserted as padding.
    return result;
}

pub fn get_first_negative(vector: &Vec<i32>) -> usize {
    let mut mindex = 0;
    let mut min = 0;
    for i in 1..vector.len() {
        if vector[i] < min {
            mindex = i;
            min = vector[i];
        }
    }
    return mindex;
}
