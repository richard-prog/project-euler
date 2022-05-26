pub fn p03() -> u64 {
    let v = get_prime_factors(600851475143);
    v[v.len() - 1]
}

fn get_prime_factors(mut x: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut n = 1;
    while n < x {
        n += 1;
        while x % n == 0 {
            v.push(n);
            x /= n;
        }
    }
    v
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
