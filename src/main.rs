mod problems;

use problems::problem::*;

fn main() {
    run_problem(problems::problem1::Problem1, 1000);
    run_problem(problems::problem2::Problem2, 4_000_000);
    run_problem(problems::problem3::Problem3, 600851475143)
}
