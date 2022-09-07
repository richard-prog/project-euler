use crate::utility;
use std::cmp::Ordering;
use std::ops::Mul;

pub fn p33() -> u64 {
    let mut fractions = Vec::with_capacity(4);
    let one = Fraction::new(1, 1);
    for a in 1..=9 {
        for b in 1..=9 {
            for c in 1..=9 {
                let major = Fraction::new(a * 10 + b, c * 10 + a);
                let minor = Fraction::new(a * 10 + b, b * 10 + c);
                if major == Fraction::new(b, c) && major < one {
                    fractions.push(major);
                }
                if minor == Fraction::new(a, c) && minor < one {
                    fractions.push(minor);
                }
            }
        }
    }
    let mut product = Fraction::new(1, 1);
    for frac in fractions {
        product = product * frac;
    }
    product.denominator
}

#[derive(Debug)]
struct Fraction {
    numerator: u64,
    denominator: u64,
}

impl Fraction {
    fn new(numerator: u64, denominator: u64) -> Fraction {
        Fraction {
            numerator,
            denominator,
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, rhs: &Self) -> bool {
        self.numerator * rhs.denominator == rhs.numerator * self.denominator
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left_product = self.numerator * other.denominator;
        let right_product = self.denominator * other.numerator;
        Some(left_product.cmp(&right_product))
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        let gcd = utility::gcd(numerator, denominator);
        Fraction::new(numerator / gcd, denominator / gcd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p33(), 100);
    }

    #[test]
    fn test_fraction_equality() {
        let two_thirds = Fraction::new(2, 3);
        let six_ninths = Fraction::new(6, 9);
        assert_eq!(two_thirds, six_ninths);
        let one_half = Fraction::new(1, 2);
        assert_ne!(one_half, two_thirds);
    }
}
