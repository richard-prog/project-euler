pub fn p06() -> u64 {
    let n: u64 = 100;
    let mut sum_of_squares = 0;
    for i in 0..=n {
        sum_of_squares += i.pow(2);
    }
    let mut square_of_sum = 0;
    for i in 0..=n {
        square_of_sum += i;
    }
    square_of_sum = square_of_sum.pow(2);
    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_solution() {
        assert_eq!(p06(), 25164150);
    }
}
