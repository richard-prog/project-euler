pub fn p07(primes: &Vec<u32>) -> u64 {
    primes[10_000] as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	use crate::primes;
        assert_eq!(p07(&primes::get_primes()), 104743);
    }
}
