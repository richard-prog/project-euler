pub fn p02() -> u64 {
    let mut cur = 0;
    let mut next = 1;
    let mut sum = 0;
    while next < 4_000_000 {
        let tmp = next;
        next += cur;
        cur = tmp;
        if cur % 2 == 0 {
            sum += cur;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(4613732, p02());
    }
}
