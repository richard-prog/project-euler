pub fn p02() -> u64 {
    sum_even_fibonacci_numbers_at_most(4_000_000)
}

fn sum_even_fibonacci_numbers_at_most(upper_limit: u64) -> u64 {
    let (mut cur, mut next) = (0, 1);
    let mut sum = 0;
    while advance_fibonacci(&mut cur, &mut next) <= upper_limit {
	if cur % 2 == 0 {
	    sum += cur;
	}
    }
    sum
}

fn advance_fibonacci(cur: &mut u64, next: &mut u64) -> u64 {
    (*cur, *next) = (*next, *cur + *next);
    *cur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_even_fibonacci_numbers_at_most() {
	assert_eq!(sum_even_fibonacci_numbers_at_most(89), 44);
	assert_eq!(sum_even_fibonacci_numbers_at_most(34), 44);
	assert_eq!(sum_even_fibonacci_numbers_at_most(33), 10);
    }

    #[test]
    fn test_advance_fibonacci() {
	let (mut cur, mut next) = (6, 8);
	assert_eq!(advance_fibonacci(&mut cur, &mut next), 8);
	assert_eq!(cur, 8);
	assert_eq!(next, 14);
    }
    
    #[test]
    fn check_solution() {
        assert_eq!(4613732, p02());
    }
}
