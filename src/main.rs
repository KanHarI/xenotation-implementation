mod num_theory;
mod xenotation;

use xenotation::XenotationCache;
use crate::num_theory::populate_primes_up_to;

fn calculate_average_representation_over_ln(cache: &mut XenotationCache, m: usize) -> f64 {
    populate_primes_up_to(m); // Populate the primes up to m (inclusive)
    let mut sum = 0.0;

    for n in 1..=m {
        let length = cache.count_xenofied_len(n) as f64; // Get the length of the representation
        let log_n = (n as f64).ln(); // Natural logarithm of n

        if log_n > 0.0 {
            sum += length / log_n; // Add the ratio to the sum
        } else {
            // Handle the case when n = 1, log(1) = 0, which would lead to division by zero
            sum += length; // Optionally, you can add just the length or some constant
        }
    }

    sum / m as f64 // Return the average
}

fn main() {
    let mut cache = XenotationCache::new(); // Create a new cache

    for i in 1..=100 {
        let xeno = cache.int_to_xenotation(i);
        let representation = XenotationCache::stringify(&xeno);
        println!("{}: {}", i, representation);
    }

    for m in [10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000].iter() {
        let average = calculate_average_representation_over_ln(&mut cache, *m);
        println!(
            "Average representation length over ln for m = 1..{}: {}",
            m, average
        );
    }
}
