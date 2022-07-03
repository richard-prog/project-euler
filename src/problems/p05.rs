pub fn p05(primes: &[u32]) -> u64 {
    smallest_divisible_by_numbers_at_most(20, primes)
}

fn smallest_divisible_by_numbers_at_most(upper_limit: u32, primes: &[u32]) -> u64 {
    let mut product = 1;
    let mut i = 0;
    while primes[i] < upper_limit {
	let base = primes[i];
	let mut partial_product = base;
	while partial_product * base < upper_limit {
	    partial_product *= base;
	}
	product *= partial_product as u64;
	i += 1;
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_solution() {
	use crate::primes;
        assert_eq!(p05(&primes::get_primes()), 232792560);
    }

    #[test]
    fn test_smallest_divisible_by_numbers_at_most() {
	use crate::primes;
        assert_eq!(smallest_divisible_by_numbers_at_most(10,
							 &primes::get_primes()),
		   2520);
    }
}
