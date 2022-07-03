const NUMBER_FOR_PROBLEM_THREE: u64 = 600851475143;

pub fn p03() -> u64 {
    *get_prime_factors(NUMBER_FOR_PROBLEM_THREE).last().unwrap()
}

fn get_prime_factors(mut upper_limit: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut factor = 1;
    while factor < upper_limit {
        factor += 1;
        while upper_limit % factor == 0 {
            factors.push(factor);
            upper_limit /= factor;
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p03(), 6857);
    }

    #[test]
    fn test_get_prime_factors() {
        let test_vec: Vec<u64> = vec![5, 7, 13, 29];
        assert_eq!(get_prime_factors(13195), test_vec);
    }
}
