//use crate::primes;
pub fn p05() -> u64 {
    get_smallest_divisible(20)
}

fn get_smallest_divisible(n: u32) -> u64 {
    let prime_vec = crate::primes::get_primes();
    let mut count_vec: Vec<(u32, u32)> = Vec::new();
    let mut i = 0;
    while prime_vec[i] < n {
        count_vec.push((prime_vec[i], 1));
        i += 1
    }
    for i in 0..count_vec.len() {
        let base = count_vec[i].0;
        let mut exp = count_vec[i].1;
        while base.pow(exp + 1) <= n {
            exp += 1;
        }
        if exp == 1 {
            break;
        }
        count_vec[i].1 = exp;
    }
    let mut product = 1u64;
    for pair in count_vec {
        product *= pair.0.pow(pair.1) as u64;
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_solution() {
        assert_eq!(p05(), 232792560);
    }

    #[test]
    fn test_get_smallest_divisible() {
        assert_eq!(get_smallest_divisible(10), 2520);
    }
}
