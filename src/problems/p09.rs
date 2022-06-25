const SUM: u64 = 1000;

pub fn p09() -> u64 {
    let mut a = SUM / 3;
    while a > 0 {
        let mut b = 1;
        while a + b < SUM {
            let c = SUM - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return a * b * c;
            }
            b += 1;
        }
        a -= 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p09(), 31875000);
    }
}
