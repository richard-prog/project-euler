//use crate::primes;

pub fn p07() -> u64 {
    crate::primes::get_primes()[10_000] as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p07(), 104743);
    }
}
