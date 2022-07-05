use std::collections::HashMap;

use crate::primes;

pub fn p29(primes: &Vec<u32>) -> u64 {
    num_distinct_terms(100, 100, primes)
}

fn num_distinct_terms(base_max: u64, exp_max: u64, primes: &Vec<u32>) -> u64{
    let bases: Vec<Vec<(u32, u32)>> = (2..=base_max)
	.map(|base| primes::factor(base, primes))
	.collect();
    let mut terms = HashMap::new();
    for factorization in &bases {
	for exp in 2..=exp_max {
	    let mut cur_factorization = factorization.clone();
	    for mut factor in &mut cur_factorization {
		factor.1 *= exp as u32;
	    }
	    terms.insert(cur_factorization, ());
	}
    }
    terms.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	let primes = primes::get_primes();
	assert_eq!(p29(&primes), 9183);
    }
    
    #[test]
    fn test_num_distinct_terms() {
	let primes = primes::get_primes();
	assert_eq!(num_distinct_terms(5, 5, &primes), 15);
    }
}
