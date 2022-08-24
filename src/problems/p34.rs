pub fn p34() -> u64 {
    let mut factorials: [u64; 10] = [1; 10];
    for i in 1..10 {
        factorials[i] = factorials[i - 1] * i as u64;
    }
    let mut digits: [u8; 7] = [0; 7];
    let upper_limit = factorials[9] * 7;
    let mut sum = 0;

    digits[5] = 1; //Trivial sums don't count, so we start at 10
    for i in 10..=upper_limit {
        if digit_factorial_sum(&digits, &factorials) == i {
            sum += i;
        }
        increment_digits(&mut digits);
    }
    sum
}

fn increment_digits(digits: &mut [u8]) {
    let l = digits.len();
    for i in 1..=l {
        if digits[l - i] != 9 {
            digits[l - i] += 1;
            break;
        }
        digits[l - i] = 0;
    }
}

fn digit_factorial_sum(digits: &[u8], factorials: &[u64]) -> u64 {
    let mut sum = 0;
    let mut i = 0;
    while digits[i] == 0 {
        i += 1;
    }
    let l = digits.len();
    while i < l {
        sum += factorials[digits[i] as usize];
        i += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p34(), 40730);
    }

    #[test]
    fn test_digit_factorial_sum() {
        let digits = [1, 4, 5];
        let mut factorials: [u64; 10] = [1; 10];
        for i in 1..10 {
            factorials[i] = factorials[i - 1] * i as u64;
        }
        assert_eq!(digit_factorial_sum(&digits, &factorials), 145);
    }
}
