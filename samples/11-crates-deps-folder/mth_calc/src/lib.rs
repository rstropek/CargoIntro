use std::thread;

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
        assert!(result.0 >= 1 && result.1 >= 1 && result.0 / result.1 == 1, "Unexpected result");
    }
}
