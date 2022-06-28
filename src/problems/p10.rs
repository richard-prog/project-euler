pub fn p10(primes: &Vec<u32>) -> u64 {
    let mut sum: u64 = 0;
    let mut i = 0;
    let l = primes.len();
    while i < l && primes[i] < 2_000_000 {
        sum += primes[i] as u64;
        i += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	use crate::primes;
        assert_eq!(p10(&primes::get_primes()), 142913828922);
    }
}
