use std::error::Error;

pub fn p09() -> Result<u64, Box<dyn Error>> {
    let sum = 1000;
    let (a, b, c) = pythagorean_triple_with_sum(sum)?;
    Ok(a * b * c)
}

fn pythagorean_triple_with_sum(sum: u64) -> Result<(u64, u64, u64), Box<dyn Error>> {
    let mut a = sum / 3;
    while a > 0 {
        let mut b = 1;
        while a + b < sum {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                if a > b {
                    (a, b) = (b, a);
                }
                return Ok((a, b, c));
            }
            b += 1;
        }
        a -= 1;
    }
    Err(format!("No triple is sum {sum} found").into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p09().unwrap(), 31875000);
    }

    #[test]
    fn test_pythagorean_triple_with_sum() {
        assert_eq!(pythagorean_triple_with_sum(12).unwrap(), (3, 4, 5));
	assert!(pythagorean_triple_with_sum(3).is_err());
    }
}
