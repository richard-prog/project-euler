pub fn p04() -> u64 {
    let mut largest_so_far = -1;
    let mut diagonal_length = 1;
    let max = 999;
    while largest_so_far < 0 {
        let min = max - diagonal_length + 1;
        for i in 0..diagonal_length {
            let product = (max - i) * (min + i);
            if is_palindrome(product) && product > largest_so_far {
                largest_so_far = product;
            }
        }
        diagonal_length += 1;
    }
    largest_so_far as u64
}

fn is_palindrome(n: i64) -> bool {
    let s = format!("{}", n);
    let digits = s.as_bytes();
    let num_digits = digits.len();
    for i in 0..num_digits {
	if digits[i] != digits[(num_digits - 1) - i] {
	    return false;
	}
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_solution() {
        assert_eq!(p04(), 906609);
    }
    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(9009), true);
        assert_eq!(is_palindrome(30203), true);
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(12), false);
        assert_eq!(is_palindrome(123), false);
    }
}
