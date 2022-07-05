const NUM_TERMS: u64 = 100;

pub fn p06() -> u64 {
    //Closed form of sum(n^2), using Horner's method
    let sum_of_squares = NUM_TERMS * ((2 * NUM_TERMS + 3) * NUM_TERMS + 1) / 6;
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
