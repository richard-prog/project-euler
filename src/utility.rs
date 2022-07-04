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
}
