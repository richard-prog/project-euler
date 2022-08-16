use std::collections::HashMap;

use crate::utility;

pub fn p27(primes: &Vec<u32>) -> i64 {
    let mut prime_hash = HashMap::new();
    for prime in primes {
        prime_hash.insert(*prime as i64, ());
    }

    let mut max_coefficients = (0, 0, 0);
    let mut num_primes = 0;
    for a in -999..=999 {
        for b in -1000..=1000 {
            let num_polynomial_primes = count_primes((1, a, b), &prime_hash);
            if num_polynomial_primes > num_primes {
                num_primes = num_polynomial_primes;
                max_coefficients = (1, a, b);
            }
        }
    }
    max_coefficients.1 * max_coefficients.2
}

fn count_primes(p: (i64, i64, i64), prime_hash: &HashMap<i64, ()>) -> u64 {
    let mut count: i64 = 0;
    while prime_hash
        .get(&utility::evaluate_quadratic(p, count))
        .is_some()
    {
        count += 1;
    }
    count as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primes;

    #[ignore]
    #[test]
    fn check_solution() {
        let primes = primes::get_primes();
        assert_eq!(p27(&primes), -59231);
    }

    #[test]
    fn test_count_primes() {
        let primes = primes::get_primes();
        let mut prime_hash = HashMap::new();
        for prime in primes {
            prime_hash.insert(prime as i64, ());
        }
        let num_primes = count_primes((1, 1, 41), &prime_hash);
        assert_eq!(num_primes, 40);
        assert_eq!(count_primes((1, 1, 4), &prime_hash), 0);
    }
}
