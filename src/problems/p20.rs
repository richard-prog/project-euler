use crate::symbolic_math;

pub fn p20() -> u64 {
    sum_factorial_digits(100)
}

fn sum_factorial_digits(n: u32) -> u64 {
    symbolic_math::sum_symbolic_digits(&symbolic_math::symbolic_factorial(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p20(), 648);
    }
    
    #[test]
    fn test_sum_factorial_digits() {
	assert_eq!(sum_factorial_digits(10), 27);
    }
}
