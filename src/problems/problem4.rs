// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is
// 9009 = 91 x 99.
//
// Find the largest palindrome made from the product of two
// 2-digit numbers.

use crate::problems::problem::Problem;

pub struct Problem4;

impl Problem<u64, u64> for Problem4 {
  fn id() -> u8 {
    4
  }

  fn run(input: u64) -> u64 {
    let mut largest_palindrome: u64 = 0;

    for i in (100u64..=999).rev() {
      // One observation is that for a 6-digit palindrome ABCBA, it can be expressed as:
      //    ABCBA
      //      =100000A+10000B+1000C+100B+10A
      //      =100001A+10010B+1100C
      //
      // This means that a palindrome of this form must be divisible by 11.
      // With this in mind, we can iterate over multiples of 11 within the 3-digit
      // range and check for factors within the 3-digit range. This eliminates the need
      // to check all combinations, significantly reducing the number of iterations.
      let step = if i % 11 == 0 { 1 } else { 11 };
      for j in (100u64..=999).rev().step_by(step) {
        let product = i * j;
        if product < largest_palindrome {
          break;
        }

        if is_palindrome(product) && product > largest_palindrome {
          largest_palindrome = product;
        }
      }
    }

    largest_palindrome
  }
}

fn is_palindrome(n: u64) -> bool {
  let s = n.to_string();
  s.chars().eq(s.chars().rev())
}