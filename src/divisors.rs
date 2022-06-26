use crate::primes;

pub fn count_divisors(num: u32, primes: &Vec<u32>) -> u32 {
    let factorization = primes::factor(num as u64, primes);
    let mut num_divisors = 1;
    for (_, exp) in factorization {
	num_divisors *= exp + 1;
    }
    num_divisors
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count_divisors() {
	assert_eq!(count_divisors(28, &primes::get_primes()), 6);
    }
}
