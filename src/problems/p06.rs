const NUM_TERMS: u64 = 100;

pub fn p06() -> u64 {
    let sum_of_squares: u64 = (1..=NUM_TERMS).map(|x| x * x).sum();
    let sum = (NUM_TERMS) * (NUM_TERMS + 1) / 2;
    let square_of_sum = sum * sum;
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
