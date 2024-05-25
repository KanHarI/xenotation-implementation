use crate::num_theory;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Xenotation {
    Two,
    Primify(Rc<Xenotation>),
    Product(Vec<Rc<Xenotation>>),
}

pub struct XenotationCache {
    cache: HashMap<usize, Rc<Xenotation>>,
    two: Rc<Xenotation>,
    lengths_cache: HashMap<usize, usize>,
}

impl XenotationCache {
    pub(crate) fn new() -> Self {
        let two = Rc::new(Xenotation::Two);
        let cache = HashMap::new();
        let lengths_cache = HashMap::new();
        XenotationCache { cache, two, lengths_cache }
    }

    pub fn int_to_xenotation(&mut self, n: usize) -> Rc<Xenotation> {
        if let Some(cached) = self.cache.get(&n) {
            return cached.clone();
        }

        let factorization = num_theory::factorize(n);
        let result = if factorization.len() == 1 && factorization.values().next().unwrap() == &1 {
            let factor = factorization.keys().next().unwrap();
            if *factor == 2 {
                self.two.clone()
            } else {
                let index = num_theory::prime_to_index(*factor);
                let prime_representation = self.int_to_xenotation(index + 1);
                Rc::new(Xenotation::Primify(prime_representation))
            }
        } else {
            let mut product = Vec::new();
            for (&prime, &power) in factorization.iter().rev() {
                let prime_representation = self.int_to_xenotation(prime);
                for _ in 0..power {
                    product.push(prime_representation.clone());
                }
            }
            Rc::new(Xenotation::Product(product))
        };

        self.cache.insert(n, result.clone());
        result
    }

    pub fn xenotation_to_int(x: &Xenotation) -> usize {
        match x {
            Xenotation::Two => 2,
            Xenotation::Primify(x) => {
                let n = XenotationCache::xenotation_to_int(x);
                num_theory::index_to_prime(n - 1)
            }
            Xenotation::Product(xs) => xs
                .iter()
                .map(|x| XenotationCache::xenotation_to_int(x))
                .product(),
        }
    }

    pub fn stringify(x: &Xenotation) -> String {
        match x {
            Xenotation::Two => ".".to_string(),
            Xenotation::Primify(x) => format!("({})", XenotationCache::stringify(x)),
            Xenotation::Product(xs) => {
                let mut result = String::new();
                for x in xs {
                    result.push_str(&XenotationCache::stringify(x));
                }
                result
            }
        }
    }

    pub fn count_stringified_len(x: &Xenotation) -> usize {
        match x {
            Xenotation::Two => 1,
            Xenotation::Primify(x) => 2 + XenotationCache::count_stringified_len(x),
            Xenotation::Product(xs) => xs.iter().map(|x| XenotationCache::count_stringified_len(x)).sum(),
        }
    }

    pub fn count_xenofied_len(&mut self, x: usize) -> usize {
        let factorization = num_theory::factorize(x);
        if factorization.len() == 1 && factorization.values().next().unwrap() == &1 {
            let factor = factorization.keys().next().unwrap();
            if *factor == 2 {
                1
            } else {
                if let Some(&cached) = self.lengths_cache.get(&x) {
                    return cached;
                }
                let index = num_theory::prime_to_index(*factor);
                let result = 2 + self.count_xenofied_len(index + 1);
                self.lengths_cache.insert(x, result);
                result
            }
        } else {
            factorization.iter().map(|(&prime, &power)| self.count_xenofied_len(prime) * power).sum()
        }
    }
}
