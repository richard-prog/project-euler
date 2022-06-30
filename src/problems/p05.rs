pub fn p05(primes: &[u32]) -> u64 {
    get_smallest_divisible(20, primes)
}

fn get_smallest_divisible(n: u32, primes: &[u32]) -> u64 {
    let mut count_vec: Vec<(u32, u32)> = Vec::new();
    let mut i = 0;
    while primes[i] < n {
        count_vec.push((primes[i], 1));
        i += 1
    }
    for (base, exp) in &mut count_vec {
        // let base = count_vec[i].0;
        // let mut exp = count_vec[i].1;
        while base.pow(*exp + 1) <= n {
            *exp += 1;
        }
        if *exp == 1 {
            break;
        }
        //count_vec[i].1 = exp;
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
	use crate::primes;
        assert_eq!(p05(&primes::get_primes()), 232792560);
    }

    #[test]
    fn test_get_smallest_divisible() {
	use crate::primes;
        assert_eq!(get_smallest_divisible(10, &primes::get_primes()), 2520);
    }
}
