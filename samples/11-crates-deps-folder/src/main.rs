use std::time::Instant;
use rand::prelude::*;
use num_format::{Locale, ToFormattedString};

// Sample calculating PI using a Monte Carlo simulation.
// Note that the goal of this sample is NOT to calculate PI
// as exactly or efficiently as possible. The goal is to 
// demonstrate various features of the Rust language.

fn main() {
    // Number of iterations per thread
    const ITERATIONS: u32 = 1000000;

    // Start stopwatch
    let now = Instant::now();

    let result = mth_calc::run_on_all_cpus::<u32>(|| {
        // Create a thread-local random generator
        let mut rng = rand::rng();

        // Count number of random points inside unit circle
        // within quarter circle segment.
        let mut inside: u32 = 0;
        for _ in 0..ITERATIONS {
            let a = rng.random::<f64>();
            let b = rng.random::<f64>();
            let c = a.powf(2f64) + b.powf(2f64);
            if c <= 1f64 {
                inside += 1;
            }
        }

        inside
    },
    0u32,
    |x, y| { x + y });

    // Calculate PI
    let total_iterations = ITERATIONS * result.1;
    let pi = result.0 as f64 / total_iterations as f64 * 4f64;
    
    // Print PI and the avg. number of calculations/sec
    println!("{}", pi);
    let calculations_string = ((total_iterations as f64 / now.elapsed().as_secs_f64()) as u32).to_formatted_string(&Locale::en);
    println!("{} calculations per second", calculations_string);
}
