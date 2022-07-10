use crate::primes;

pub fn count_divisors(num: u32, primes: &Vec<u32>) -> u32 {
    let factorization = primes::factor(num as u64, primes);
    let mut num_divisors = 1;
    for (_, exp) in factorization {
        num_divisors *= exp + 1;
    }
    num_divisors
}

pub fn get_proper_divisors(num: u64, primes: &Vec<u32>) -> Vec<u32> {
    let factorization = primes::factor(num, primes);
    let mut capacity = 1;
    for (_, exp) in &factorization {
        capacity *= exp + 1;
    }
    let mut divisors = Vec::with_capacity(capacity as usize + 1);
    divisors.push(1);
    for (base, exp) in factorization {
        extend_divisors(&mut divisors, base, exp);
    }
    divisors.pop();
    divisors
}

pub fn sum_proper_divisors(num: u64, primes: &Vec<u32>) -> u32 {
    if num == 0 {
        return 0;
    }
    get_proper_divisors(num, primes).iter().sum()
}

fn extend_divisors(divisors: &mut Vec<u32>, base: u32, exponent: u32) {
    let n = divisors.len();
    let mut multiple = base;
    for _ in 1..=exponent {
        for j in 0..n {
            divisors.push(divisors[j] * multiple);
        }
        multiple *= base;
    }
    //Want sort instead of sort_unstable because we're adding a sorted slice
    divisors.sort_unstable();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_divisors() {
        assert_eq!(count_divisors(28, &primes::get_primes()), 6);
    }

    #[test]
    fn test_extend_divisors() {
        let mut divisors = vec![1, 3, 9];
        extend_divisors(&mut divisors, 2, 3);
        divisors.sort();
        assert_eq!(divisors, vec![1, 2, 3, 4, 6, 8, 9, 12, 18, 24, 36, 72]);
    }

    #[test]
    fn test_get_proper_divisors() {
        assert_eq!(
            get_proper_divisors(220, &primes::get_primes()),
            vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]
        );
        assert_eq!(
            get_proper_divisors(284, &primes::get_primes()),
            vec![1, 2, 4, 71, 142]
        );
        assert_eq!(
            get_proper_divisors(1, &primes::get_primes()),
            Vec::<u32>::new()
        );
    }

    #[test]
    fn test_sum_proper_divisors() {
        let primes = primes::get_primes();
        assert_eq!(sum_proper_divisors(220, &primes), 284);
        assert_eq!(sum_proper_divisors(284, &primes), 220);
        assert_eq!(sum_proper_divisors(1, &primes), 0);
    }
}
