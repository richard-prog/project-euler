pub fn p09() -> u64 {
    let (a, b, c) = pythagorean_triple_with_sum(1000).unwrap();
    a * b * c
}

fn pythagorean_triple_with_sum(sum: u64) -> Option<(u64, u64, u64)> {
    let mut a = sum / 3;
    while a > 0 {
        let mut b = 1;
        while a + b < sum {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                return Some((a, b, c))
            }
            b += 1;
        }
        a -= 1;
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p09(), 31875000);
    }
}
