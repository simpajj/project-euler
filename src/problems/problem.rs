use colored::*;
use std::fmt::{Debug, Display};
use std::time::{Duration, Instant};

pub trait Problem<I, O> {
    fn id() -> u8;
    fn run(input: I) -> O;
}

pub fn run_problem<P, I, O>(_: P, input: I)
where
    P: Problem<I, O>,
    O: Debug,
{
    let problem_id = P::id();

    // Setup timer
    let start = Instant::now();
    // Run
    let output = P::run(input);
    // Get elapsed time
    let elapsed = start.elapsed();

    // Print result

    println!(
        "Problem {}: result = {:?}, time = {}",
        problem_id,
        output,
        fmt_time(elapsed)
    );
}

fn fmt_time(t: Duration) -> impl Display {
    let res = format! {"{:?}", t};
    if t < Duration::from_micros(100) {
        res.bright_green()
    } else if t < Duration::from_millis(1) {
        res.green()
    } else if t < Duration::from_millis(10) {
        res.bright_red()
    } else {
        res.red()
    }
}
