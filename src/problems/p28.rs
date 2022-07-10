pub fn p28() -> u64 {
    sum_of_diagonals(1001)
}

fn sum_of_diagonals(dim: u64) -> u64 {
    let n = dim / 2;
    //Horner's method for 16n^2 + 30n + 26
    1 + n * ((16 * n + 30) * n + 26) / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_diagonals() {
        assert_eq!(sum_of_diagonals(5), 101);
    }

    #[test]
    fn check_solution() {
        assert_eq!(p28(), 669171001);
    }
}
