# A simple mutlithreaded Stochastic Local Search SAT solver
This project is a simple rust implementation of an SLS SAT solver.
In this implementation, a random assignment is iteratively improved upon until either a solution is found, or a local minimum is detected.
If a local minimum is found, then the assignment is slightly altered and the process is repeated.
The degree to which the assignment is altered varies based how many times we have gotten stuck in local minima.
If this happens too often, the process is automatically restarted.
This process is done concurrently on all available processors until a solution is found.

Note that this will never stop for unsolvable formulas.
If you are uncertain whether or not the formula given to the solver is satisfiable, then make sure to use a timeout.


## Installation
This is a cargo project, so you therefore need to first install cargo.
Luckilly, cargo is packaged with Rust, so installing Rust should do nicely.

Next ``cd`` into the project directory and run
```
cargo check
```
to install required dependancies and check that everything works nicely.
To run the program use
```
cargo run -r <formula file> <output file> <numthreads (default=num available threads)>
```

## Input ouput format
The formula provided to the program must be in DIMACS form
The output of the program is written to the given output file.
Read more in www.satcompetition.org/2004/format-solvers2004.html 
