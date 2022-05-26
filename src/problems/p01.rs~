pub fn p01() -> u64 {
    let relevant_multiple = |x| x % 3 == 0 || x % 5 == 0;
    let mut sum = 0;
    for n in 1..1000 {
        if relevant_multiple(n) {
            sum += n;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(233168, p01());
    }
}
