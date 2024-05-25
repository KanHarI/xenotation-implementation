use crate::num_theory;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Xenotation {
    Two,
    Primify(Rc<Xenotation>),
    Product(Vec<Rc<Xenotation>>),
}

impl Xenotation {
    pub fn int_to_xenotation(n: usize) -> Xenotation {
        let factorization = num_theory::factorize(n);
        // Check if n is prime
        if factorization.len() == 1 && factorization.values().next().unwrap() == &1 {
            let factor = factorization.keys().next().unwrap();
            if *factor == 2 {
                Xenotation::Two
            } else {
                let prime_representation = Rc::new(Xenotation::int_to_xenotation(
                    num_theory::prime_to_index(*factor) + 1,
                ));
                Xenotation::Primify(prime_representation)
            }
        } else {
            let mut product = Vec::new();
            let mut primes: Vec<_> = factorization.keys().cloned().collect();
            primes.sort();
            for prime in primes.iter().rev() {
                let power = factorization.get(prime).unwrap();
                let prime_representation = Rc::new(Xenotation::int_to_xenotation(prime.clone()));
                for _ in 0..*power {
                    product.push(prime_representation.clone());
                }
            }
            Xenotation::Product(product)
        }
    }

    pub fn xenotation_to_int(x: &Xenotation) -> usize {
        match x {
            Xenotation::Two => 2,
            Xenotation::Primify(x) => {
                let n = Xenotation::xenotation_to_int(x);
                num_theory::index_to_prime(n - 1)
            }
            Xenotation::Product(xs) => xs
                .iter()
                .map(|x| Xenotation::xenotation_to_int(x))
                .product(),
        }
    }

    pub fn stringify(x: &Xenotation) -> String {
        match x {
            Xenotation::Two => ".".to_string(),
            Xenotation::Primify(x) => format!("({})", Xenotation::stringify(x)),
            Xenotation::Product(xs) => {
                let mut result = String::new();
                for x in xs {
                    result.push_str(&Xenotation::stringify(x));
                }
                result
            }
        }
    }
}
