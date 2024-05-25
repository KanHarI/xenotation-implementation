use lazy_static::lazy_static;
use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

struct PrimeCache {
    primes: Vec<usize>,
    prime_to_index: HashMap<usize, usize>,
}

lazy_static! {
    static ref PRIME_CACHE: Mutex<PrimeCache> = Mutex::new(PrimeCache {
        primes: vec![2, 3],
        prime_to_index: {
            let mut m = HashMap::new();
            m.insert(2, 0);
            m.insert(3, 1);
            m
        },
    });
}

fn _is_prime(n: usize, primes: &[usize], cache_populated_up_to: &usize, primes_to_idx: &HashMap<usize, usize>) -> bool {
    if cache_populated_up_to >= &n {
        return primes_to_idx.contains_key(&n);
    }
    for &prime in primes {
        if prime * prime > n {
            break;
        }
        if n % prime == 0 {
            return false;
        }
    }
    true
}

pub fn populate_primes_up_to(n: usize) {
    let mut cache = PRIME_CACHE.lock().unwrap();
    let mut candidate = cache.primes.last().unwrap() + 2;
    while candidate <= n {
        let primes = cache.primes.clone(); // Clone the list of primes
        let primes_to_idx = &cache.prime_to_index;
        let cache_populated_up_to = cache.primes.last().unwrap();
        if _is_prime(candidate, &primes, cache_populated_up_to, primes_to_idx) {
            let prime_len = cache.primes.len();
            cache.prime_to_index.insert(candidate, prime_len);
            cache.primes.push(candidate);
        }
        candidate += 2;
    }
}

pub fn populate_primes_till_count(n: usize) {
    let mut cache = PRIME_CACHE.lock().unwrap();
    let mut candidate = cache.primes.last().unwrap() + 2;
    while cache.primes.len() < n {
        let primes = cache.primes.clone(); // Clone the list of primes
        let primes_to_idx = &cache.prime_to_index;
        let cache_populated_up_to = cache.primes.last().unwrap();
        if _is_prime(candidate, &primes, cache_populated_up_to, primes_to_idx) {
            let prime_len = cache.primes.len();
            cache.prime_to_index.insert(candidate, prime_len);
            cache.primes.push(candidate);
        }
        candidate += 2;
    }
}

pub fn is_prime(n: usize) -> bool {
    let max_relevant_prime = (n as f64).sqrt().ceil() as usize;
    populate_primes_up_to(max_relevant_prime);
    let cache = PRIME_CACHE.lock().unwrap();
    let primes_to_idx = &cache.prime_to_index;
    let cache_populated_up_to = cache.primes.last().unwrap();
    _is_prime(n, &PRIME_CACHE.lock().unwrap().primes, cache_populated_up_to, primes_to_idx)
}

pub fn index_to_prime(index: usize) -> usize {
    populate_primes_till_count(index + 1);
    let cache = PRIME_CACHE.lock().unwrap();
    cache.primes[index]
}

pub fn prime_to_index(prime: usize) -> usize {
    populate_primes_up_to(prime);
    let cache = PRIME_CACHE.lock().unwrap();
    *cache.prime_to_index.get(&prime).unwrap()
}

pub fn factorize(n: usize) -> BTreeMap<usize, usize> {
    let mut factors = BTreeMap::new();
    let mut n = n;
    let max_relevant_prime = (n as f64).sqrt().ceil() as usize;
    populate_primes_up_to(max_relevant_prime);
    let cache = PRIME_CACHE.lock().unwrap();
    for &prime in &cache.primes {
        if prime * prime > n {
            break;
        }
        while n % prime == 0 {
            *factors.entry(prime).or_insert(0) += 1;
            n /= prime;
        }
    }
    if n > 1 {
        *factors.entry(n).or_insert(0) += 1;
    }
    factors
}
