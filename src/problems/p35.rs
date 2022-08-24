pub fn p35(prime_sieve: &[bool], prime_vec: &[u32]) -> u64 {
    let (mut upper_limit, mut num_digits) = (10, 1);
    let mut count = 0;
    let mut i = 0;
    let mut cur_prime = prime_vec[i] as usize;
    while upper_limit <= 1_000_000 {
        while cur_prime < upper_limit {
            if check_circular_prime(cur_prime, num_digits, prime_sieve) {
                count += 1;
            }
            i += 1;
            cur_prime = prime_vec[i] as usize;
        }
        upper_limit *= 10;
        num_digits += 1;
    }
    count
}

fn check_circular_prime(mut num: usize, num_digits: u64, prime_sieve: &[bool]) -> bool {
    let mut highest_place = 1;
    for _ in 0..num_digits - 1 {
        highest_place *= 10;
    }
    for _ in 0..num_digits {
        if !prime_sieve[num] {
            return false;
        }
        let ones_digit = num % 10;
        num = num / 10 + highest_place * ones_digit;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primes;

    #[test]
    fn check_solution() {
        let (prime_sieve, prime_vec) = primes::generate_primes();
        assert_eq!(p35(&prime_sieve, &prime_vec), 55);
    }

    #[test]
    fn test_check_circular_prime() {
        let (prime_sieve, _) = primes::generate_primes();
        assert!(check_circular_prime(2, 1, &prime_sieve));
        assert!(check_circular_prime(11, 2, &prime_sieve));
        assert!(check_circular_prime(97, 2, &prime_sieve));
        assert!(check_circular_prime(197, 3, &prime_sieve));
        assert!(!check_circular_prime(23, 2, &prime_sieve));
    }
}
