mod num_theory;
mod xenotation;

use xenotation::Xenotation;

fn calculate_average_representation_over_ln(m: usize) -> f64 {
    let mut sum = 0.0;

    for n in 1..=m {
        let xeno = Xenotation::int_to_xenotation(&n);  // Convert n to its Xenotation form
        let representation = Xenotation::stringify(&xeno);  // Get string representation
        let length = representation.len() as f64;  // Convert length to f64
        let log_n = (n as f64).ln();  // Natural logarithm of n

        if log_n > 0.0 {
            sum += length / log_n;  // Add the ratio to the sum
        } else {
            // Handle the case when n = 1, log(1) = 0, which would lead to division by zero
            sum += length;  // Optionally, you can add just the length or some constant
        }
    }

    sum / m as f64  // Return the average
}


fn main() {
    for i in 1..=100 {
        let xeno = Xenotation::int_to_xenotation(&i);
        let representation = Xenotation::stringify(&xeno);
        println!("{}: {}", i, representation);
    }
    for m in [10, 100, 1000, 10000, 100000, 1000000].iter() {
        let average = calculate_average_representation_over_ln(*m);
        println!("Average representation length over ln for m = 1..{}: {}", m, average);
    }
}
