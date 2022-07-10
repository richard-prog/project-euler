pub fn p02() -> u64 {
    sum_even_fibonacci_numbers_at_most(4_000_000)
}

fn sum_even_fibonacci_numbers_at_most(upper_limit: u64) -> u64 {
    EvenFibLimit::new(upper_limit).sum()
}

struct EvenFibLimit {
    cur: u64,
    next: u64,
    limit: u64,
}

impl EvenFibLimit {
    fn new(limit: u64) -> Self {
        EvenFibLimit {
            cur: 0,
            next: 2,
            limit,
        }
    }
}

impl Iterator for EvenFibLimit {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next > self.limit {
            None
        } else {
            (self.cur, self.next) = (self.next, self.cur + 4 * self.next);
            Some(self.cur)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_even_fibonacci_numbers_at_most() {
        assert_eq!(sum_even_fibonacci_numbers_at_most(2), 2);
        assert_eq!(sum_even_fibonacci_numbers_at_most(89), 44);
        assert_eq!(sum_even_fibonacci_numbers_at_most(34), 44);
        assert_eq!(sum_even_fibonacci_numbers_at_most(33), 10);
    }

    #[test]
    fn check_solution() {
        assert_eq!(4613732, p02());
    }
}
