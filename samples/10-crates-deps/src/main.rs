use std::thread;
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

    // Start one thread per CPU
    let num_cpus = num_cpus::get() as u32;
    let mut handles = vec!();
    for _ in 0..num_cpus {
        handles.push(thread::spawn(|| {
            // Create a thread-local random generator
            let mut rng = thread_rng();

            // Count number of random points inside unit circle
            // within quarter circle segment.
            let mut inside: u32 = 0;
            for _ in 0..ITERATIONS {
                let a = rng.gen::<f64>();
                let b = rng.gen::<f64>();
                let c = a.powf(2f64) + b.powf(2f64);
                if c <= 1f64 {
                    inside += 1;
                }
            }

            inside
        }));
    }

    // Wait for all threads to complete and summarize
    // their results.
    let mut total_inside: u32 = 0;
    for h in handles {
        total_inside += h.join().unwrap();
    }

    // Calculate PI
    let total_iterations = ITERATIONS * num_cpus;
    let pi = total_inside as f64 / total_iterations as f64 * 4f64;
    
    // Print PI and the avg. number of calculations/sec
    println!("{}", pi);
    let calculations_string = ((total_iterations as f64 / now.elapsed().as_secs_f64()) as u32).to_formatted_string(&Locale::en);
    println!("{} calculations per second", calculations_string);
}
