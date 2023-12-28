// The prime factors of 13195 are 5, 7, 13 and 29.
//
// .
//
// What is the largest prime factor of the number 600851475143?

use num::integer::gcd;
use rand::Rng;
use crate::problems::problem::Problem;

pub struct Problem3;

impl Problem<u64, u64> for Problem3 {
  fn id() -> u8 {
    3
  }

  fn run(input: u64) -> u64 {
    largest_prime_factor(input)
  }
}

fn largest_prime_factor(mut n: u64) -> u64 {
  let mut factor = 2;

  while factor * factor <= n {
    if n % factor == 0 {
      n /= factor;
    } else {
      factor += 1;
    }
  }

  n
}