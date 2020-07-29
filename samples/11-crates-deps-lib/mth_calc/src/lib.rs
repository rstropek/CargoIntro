use std::thread;

/// Runs a given closure in parallel on all available CPUs.
/// 
/// The given closure can return a value. All return values of all closure executions are 
/// aggregated with a given aggregation closure. The method returns a tuple with
/// the aggregated result and the number of executions (=number of CPUs).
/// 
/// # Arguments
/// 
/// * `body` - closure that should be run in parallel on all available processors.
/// * `initial` - initial value used for aggregating the results of all closure executions.
/// * `agg` - closure used to aggregate method results.
/// 
/// # Examples
/// 
/// ```
/// use mth_calc::run_on_all_cpus;
/// use rand::prelude::*;
/// 
/// let result = mth_calc::run_on_all_cpus::<f32>(|| {
///     // Create a thread-local random generator
///     let mut rng = thread_rng();
///
///     // Calculate sum over ten random numbers
///     let mut total = 0f32;
///     for _ in 0..10 {
///         total += rng.gen::<f32>();
///     }
///
///     total
/// }, 
/// 0f32, // initial value for aggreation is zero
/// |x, y| { x + y }); // aggregation function = sum
/// 
/// println!("The average random number was {}", result.0 / (result.1 * 10) as f32);
/// ```
pub fn run_on_all_cpus<T: Send + 'static>(body: fn() -> T, initial: T, agg: fn(T, T) -> T) -> (T, u32) {
    // Start one thread per CPU
    let num_cpus = num_cpus::get() as u32;
    let mut handles = vec!();
    for _ in 0..num_cpus {
        handles.push(thread::spawn(body));
    }

    // Wait for all threads to complete and summarize
    // their results.
    let mut result = initial;
    for h in handles {
        result = agg(result, h.join().unwrap());
    }

    (result, num_cpus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_result_correctly() {
        let result = run_on_all_cpus(|| { 1 }, 0, |a, b| { a + b });
        if result.0 < 1 || result.1 < 1 || result.0 / result.1 != 1 {
            panic!();
        }
    }
}