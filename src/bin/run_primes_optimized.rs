use std::time::Instant;

/// Checks if a given number `n` is prime using an optimized algorithm.
///
/// This function first handles small values and simple cases:
/// - Returns `false` for numbers less than or equal to 1.
/// - Returns `true` for 2 and 3.
/// - Returns `false` for even numbers and multiples of 3.
///
/// For larger numbers, it checks divisibility by numbers of the form 6k ± 1 up to the square root of `n`.
///
/// # Arguments
///
/// * `n` - The number to check for primality.
///
/// # Returns
///
/// * `true` if `n` is prime, `false` otherwise.
///
fn is_prime(n: u64) -> bool {
    // Handle numbers less than or equal to 1 (not prime)
    if n <= 1 {
        return false;
    }

    // 2 and 3 are prime numbers
    if n == 2 || n == 3 {
        return true;
    }

    // Eliminate even numbers and multiples of 3
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    // Calculate the integer square root of n
    let sqrt_n = (n as f64).sqrt() as u64;

    // Check divisibility by numbers of the form 6k ± 1
    let mut i = 5;
    while i <= sqrt_n {
        // If n is divisible by i or i+2, it's not prime
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        // Increment by 6 to check next possible divisors
        i += 6;
    }
    // If no divisors found, n is prime
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

    println!("For n={} we found {} primes in {:.2}", n, total_primes, ms)
}
