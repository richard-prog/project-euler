use crate::divisors;

pub fn p12(primes: &Vec<u32>) -> u64 {
    let mut triangle_number = 0;
    for i in 1..u32::MAX {
	triangle_number += i;
	let num_divisors = divisors::count_divisors(triangle_number, primes);
	if num_divisors > 500 {
	    break;
	}
    }
    triangle_number as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	use crate::primes;
	assert_eq!(p12(&primes::get_primes()), 76576500);
    }
}
