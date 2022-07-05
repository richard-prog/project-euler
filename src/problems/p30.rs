use crate::utility;

pub fn p30() -> u64 {
    sum_surprising_numbers(5)
}

fn sum_surprising_numbers(exp: u32) -> u64 {
    let max = (exp as u64 + 1) * 9_u64.pow(exp);
    let mut sum: u64 = 0;
    for i in 10..=max {
	if sum_powers_of_digits(i, exp) == i {
	    sum += i;
	}
    }
    sum
}

fn sum_powers_of_digits(num: u64, pow: u32) -> u64 {
    let digits = utility::digits(num);
    digits.iter()
	.map(|digit| (*digit as u64).pow(pow))
	.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p30(), 443839);
    }

    #[test]
    fn check_sum_surprising_numbers() {
	assert_eq!(sum_surprising_numbers(4), 19316);
    }

    #[test]
    fn test_sum_powers_of_digits() {
	assert_eq!(sum_powers_of_digits(1634, 4), 1634);
	assert_eq!(sum_powers_of_digits(25, 2), 29);
    }
}
