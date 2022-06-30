use crate::divisors;

pub fn p21(primes: &Vec<u32>) -> u64 {
    sum_amicable_numbers_below(10_000, primes)
}

fn sum_amicable_numbers_below(limit: usize, primes: &Vec<u32>) -> u64 {
    let mut result: Vec<Option<bool>> = vec![None; limit + 1];
    let mut sum: u64 = 0;
    for i in 1..=limit {
	if result[i] == None {
	    match get_amicus(i as u64, primes) {
		PotentialAmicus::Amicus(j) => {
		    if j as usize <= limit {
			sum += j;
			result[j as usize] = Some(true);
		    }
		    sum += i as u64;
		    result[i as usize] = Some(true);
		}
		PotentialAmicus::Not(j) => {
		    result[i as usize] = Some(false);
		    if j as usize <= limit {
			result[j as usize] = Some(false);
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
    if potential_amicus == num {
	return PotentialAmicus::Not(potential_amicus);
    }
    else if divisors::sum_proper_divisors(potential_amicus, primes) as u64 == num {
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
