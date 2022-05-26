pub fn p10() -> u64 {
    let prime_vec = crate::primes::get_primes();
    let mut sum: u64 = 0;
    let mut i = 0;
    let l = prime_vec.len();
    while i < l && prime_vec[i] < 2_000_000 {
        sum += prime_vec[i] as u64;
        i += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p10(), 142913828922);
    }
}
