use crate::primes;
use crate::divisors;

pub fn p12() -> u64 {
    let primes = primes::get_primes();
    let mut triangle_number = 0;
    for i in 1..u32::MAX {
	triangle_number += i;
	let num_divisors = divisors::count_divisors(triangle_number, &primes);
	// println!("{triangle_number}: {num_divisors}");
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
	assert_eq!(p12(), 76576500);
    }
}
