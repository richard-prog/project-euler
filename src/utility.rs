pub fn evaluate_quadratic((a, b, c): (i64, i64, i64), x: i64) -> i64 {
    ((a * x) + b) * x + c
}

pub fn evaluate_polynomial(coefficients: &Vec<i64>, argument: i64) -> i64 {
    //Horner's algorithm
    let mut result = 0;
    for coefficient in coefficients {
	result = result * argument + coefficient;
    }
    result
}

pub fn digits(mut num: u64) -> Vec<u8> {
    if num == 0 {
	return vec![0];
    }
    let mut result = Vec::with_capacity(20);
    while num > 0 {
	result.push((num % 10) as u8);
	num /= 10;
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_polynomial() {
	assert_eq!(evaluate_polynomial(&vec![3, 4, 5], 7), 180);
	assert_eq!(evaluate_polynomial(&vec![], 8), 0);
    }

    #[test]
    fn test_evaluate_quadratic() {
	assert_eq!(evaluate_quadratic((3, 4, 5), 7), 180);
    }

    #[test]
    fn test_digits() {
	assert_eq!(digits(0), vec![0]);
	assert_eq!(digits(10043), vec![1, 0, 0, 4, 3]);
    }
}
