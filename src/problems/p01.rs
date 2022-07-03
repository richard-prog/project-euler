pub fn p01() -> u64 {
    let upper_limit = 1000;
    sum_positive_multiples(upper_limit, 3)
	+ sum_positive_multiples(upper_limit, 5)
	- sum_positive_multiples(upper_limit, 15)
}

fn sum_positive_multiples(upper_limit: u64, multiple: u64) -> u64 {
    let scaled_upper_limit = (upper_limit - 1) / multiple;
    scaled_upper_limit * (scaled_upper_limit + 1) / 2 * multiple
}
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(233168, p01());
    }

    #[test]
    fn test_sum_positive_multiples() {
	assert_eq!(sum_positive_multiples(10, 3), 18);
	assert_eq!(sum_positive_multiples(9, 3), 9);
	assert_eq!(sum_positive_multiples(1000, 3),
		   (1..1000).map(|x| -> u64 {
		       if x % 3 == 0 {
			   return x;
		       }
		       0
		   })
		   .sum::<u64>()
		   );
    }
}
