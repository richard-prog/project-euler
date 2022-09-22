pub fn p37(prime_sieve: &[bool], prime_vec: &[u32]) -> u64 {
    let mut truncatable_primes = [0; 11];
    let mut i = 0;
    let mut j = 4;
    while i < 11 {
        let cur_prime = prime_vec[j];
        if is_truncatable(cur_prime, prime_sieve) {
            truncatable_primes[i] = cur_prime;
            i += 1;
        }
        j += 1;
    }
    truncatable_primes.iter().sum::<u32>() as u64
}

fn is_truncatable(mut num: u32, prime_sieve: &[bool]) -> bool {
    let mut mask = 10;
    while mask < num {
        let remainder = num % mask;
        if !prime_sieve[remainder as usize] {
            return false;
        }
        mask *= 10;
    }
    while num > 0 {
        if !prime_sieve[num as usize] {
            return false;
        }
        num /= 10;
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
        assert_eq!(p37(&prime_sieve, &prime_vec), 748317);
    }

    #[test]
    fn test_is_truncatable() {
        let (prime_sieve, _) = primes::generate_primes();
        assert!(is_truncatable(3797, &prime_sieve));
        assert!(!is_truncatable(11, &prime_sieve));
    }
}
