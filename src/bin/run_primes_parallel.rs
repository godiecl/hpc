use std::time::Instant;

/// Checks if a given number `n` is prime using an optimized algorithm.
///
/// This function first handles small values and simple cases:
/// - Returns `false` for numbers less than or equal to 1.
/// - Returns `true` for 2 and 3.
/// - Returns `false` for even numbers and multiples of 3.
///
/// For larger numbers, it checks divisibility by numbers of the form 6k ± 1
/// up to the square root of `n`.
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
    for value in (100_000..=10_000_000).step_by(100_000) {
        // Upper limit for prime search
        let n: u64 = value;
        // println!("Finding primes to {} ..", n);

        // Threads count primes using parallel approach
        let num_threads: usize = num_cpus::get();
        // println!("Using {} threads.", num_threads);

        // Calculate chunk size for each thread
        let chunk_size = (n as usize / num_threads) + 1;
        // println!("Chunk size: {}.", chunk_size);

        // Record the start time for performance measurement
        let start = Instant::now();

        // Spawn threads to count primes in parallel.
        let mut handles = Vec::with_capacity(num_threads);

        // For each thread, assign a chunk of the range to process.
        // This loop divides the total range [2, n] into `num_threads` chunks.
        // Each thread will:
        //   1. Receive its own start and end indices.
        //   2. Count the number of primes in its assigned chunk.
        //   3. Return the count to the main thread.
        for i in 0..num_threads {
            // Start index for this thread's chunk (starts at 2)
            let start = i * chunk_size + 2;

            // End index for this thread's chunk (capped at n)
            let end = ((i + 1) * chunk_size + 1).min(n as usize);

            // Spawn a new thread to process this chunk.
            // The thread executes the closure, counting primes in [start, end].
            let handle = std::thread::spawn(move || {
                let mut count = 0;
                // Iterate through the assigned chunk and count primes.
                for num in start..=end {
                    if is_prime(num as u64) {
                        count += 1;
                    }
                }
                return count;
            });
            // Store the thread handle for later joining.
            handles.push(handle);
        }

        // Collect results from all threads.
        // Each thread returns its count, which is summed to get the total.
        let mut total_primes = 0;
        for handle in handles {
            // Join each thread and add its result to the total.
            total_primes += handle.join().unwrap();
        }

        // Measure elapsed time for the naive prime counting
        let duration = start.elapsed();

        // Convert durations to milliseconds for easier comparison
        let ms = duration.as_secs_f64() * 1000.0;

        println!(
            "For n={} we found {} primes in {:.2} ms.",
            n, total_primes, ms
        )
    }
}
