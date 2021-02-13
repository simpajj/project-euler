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

// Unsafe and will crash when computed value exceeds usize::MAX
fn sum_n(n: usize, d: usize) -> usize {
    let n = n / d;
    return d * n * (n + 1) / 2;
}
