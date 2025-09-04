use std::time::Instant;

/// Checks whether a given number `n` is prime.
///
/// This is a naive implementation that tests divisibility from 2 up to n-1.
/// For large `n`, this is inefficient, but suitable for demonstration purposes.
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // Upper limit for prime search
    let n: u64 = 600_000;
    println!("Finding primes to {} ..", n);

    // Record the start time for performance measurement
    let start = Instant::now();

    // Count primes using the naive approach
    let mut total_primes = 0;
    for i in 2..n {
        if is_prime(i) {
            total_primes += 1;
        }
    }

    // Measure elapsed time for the naive prime counting
    let duration = start.elapsed();

    // Convert durations to milliseconds for easier comparison
    let ms = duration.as_secs_f64() * 1000.0;

    println!("For n={} we found {} primes in {:.2} ms.", n, total_primes, ms)
}
