use crate::divisors;

pub fn p21(primes: &Vec<u32>) -> u64 {
    sum_amicable_numbers_below(10_000, primes)
}

fn sum_amicable_numbers_below(upper_limit: usize, primes: &Vec<u32>) -> u64 {
    let mut already_checked: Vec<bool> = vec![false; upper_limit + 1];
    let mut sum: u64 = 0;
    for i in 1..=upper_limit {
	if !already_checked[i] {
	    match get_amicus(i as u64, primes) {
		PotentialAmicus::Amicus(j) => {
		    if j as usize <= upper_limit {
			sum += j;
			already_checked[j as usize] = true;
		    }
		    sum += i as u64;
		    already_checked[i as usize] = true;
		}
		PotentialAmicus::Not(j) => {
		    already_checked[i as usize] = true;
		    if j as usize <= upper_limit {
			already_checked[j as usize] = true;
		    }
		}
	    }
	}
    }
    sum
}

#[derive(Debug, PartialEq)]
enum PotentialAmicus {
    Amicus(u64),
    Not(u64)
}

fn get_amicus(num: u64, primes: &Vec<u32>) -> PotentialAmicus {
    let potential_amicus = divisors::sum_proper_divisors(num, primes) as u64;
    let amicus_amicus = divisors::sum_proper_divisors(potential_amicus, primes);
    if amicus_amicus as u64 == num {
	if potential_amicus == num {
    	    return PotentialAmicus::Not(potential_amicus);
	}
	return PotentialAmicus::Amicus(potential_amicus);
    }
    PotentialAmicus::Not(potential_amicus)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primes;

    #[test]
    fn check_solution() {
	assert_eq!(p21(&primes::get_primes()), 31626);
    }

    #[test]
    fn test_get_amicus() {
	let primes = primes::get_primes();
	assert_eq!(get_amicus(220, &primes), PotentialAmicus::Amicus(284));
	assert_eq!(get_amicus(6, &primes), PotentialAmicus::Not(6));
	assert_eq!(get_amicus(5, &primes), PotentialAmicus::Not(1));
    }
}
