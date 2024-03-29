pub fn p25() -> u64 {
    smallest_fib_with_length(1000)
}

fn smallest_fib_with_length(len: u32) -> u64 {
    let sqrt_5 = 5f64.sqrt();
    let phi = (1f64 + sqrt_5) / 2f64;
    ((len - 1) as f64 / phi.log(10f64) + sqrt_5.log(phi)).ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p25(), 4782);
    }

    #[test]
    fn test_smallest_fib_with_length() {
        assert_eq!(smallest_fib_with_length(3), 12);
    }
}
