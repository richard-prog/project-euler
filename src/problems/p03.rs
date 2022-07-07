use crate::primes;

const NUMBER_FOR_PROBLEM_THREE: u64 = 600851475143;

pub fn p03(prime_vec: &Vec<u32>) -> u64 {
    primes::factor(NUMBER_FOR_PROBLEM_THREE, prime_vec).last().unwrap().0 as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	let prime_vec = primes::get_primes();
        assert_eq!(p03(&prime_vec), 6857);
    }
}
