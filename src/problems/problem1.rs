// If we list all the natural numbers below 10 that are multiples of 3 or 5, we
// get 3, 5, 6 and 9. The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.

use crate::problems::problem::Problem;

const M: usize = 3;
const N: usize = 5;

pub struct Problem1;

impl Problem<usize, usize> for Problem1 {
    fn id() -> u8 {
        1
    }

    fn run(input: usize) -> usize {
        sum_n(input - 1, M) + sum_n(input - 1, N) - sum_n(input - 1, M * N)
    }
}

// Unsafe and will panic when computed value exceeds usize::MAX
fn sum_n(n: usize, d: usize) -> usize {
    let n = n / d;
    return d * n * (n + 1) / 2;
}
