const UPPER_LIMIT: u64 = 28123;

use crate::divisors;

pub fn p23(primes: &Vec<u32>) -> u64 {
    let abundants = get_small_abundants(UPPER_LIMIT, primes);
    let mut nums = [true; UPPER_LIMIT as usize + 1];

    let mut loop_counter = 0;
    for (i, m) in abundants.iter().enumerate() {
        for n in abundants.iter().take(i + 1) {
            loop_counter += 1;
            let index = m + n;
            if index < UPPER_LIMIT + 1 {
                nums[index as usize] = false;
            } else {
                break;
            }
        }
    }

    nums.iter().enumerate().fold(0, |cur, (i, &num)| -> u64 {
        if num {
            return cur + i as u64;
        }
        cur
    })
}

fn get_small_abundants(lim: u64, primes: &Vec<u32>) -> Vec<u64> {
    (1..=lim)
        .filter(|n| is_abundant(*n, primes))
        .collect::<Vec<u64>>()
}

fn is_abundant(num: u64, primes: &Vec<u32>) -> bool {
    divisors::sum_proper_divisors(num, primes) as u64 > num
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primes;
    #[ignore]
    #[test]
    fn check_solution() {
        let primes = primes::get_primes();
        assert_eq!(p23(&primes), 4179871);
    }

    #[test]
    fn test_is_abundant() {
        let primes = primes::get_primes();
        assert_eq!(is_abundant(12, &primes), true);
        assert_eq!(is_abundant(6, &primes), false);
        assert_eq!(is_abundant(10, &primes), false);
    }

    #[test]
    fn test_get_small_abundants() {
        let primes = primes::get_primes();
        assert_eq!(get_small_abundants(13, &primes), vec![12]);
    }
}
