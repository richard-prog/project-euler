use crate::divisors;

pub fn p12(prime_vec: &Vec<u32>) -> u64 {
    first_triangle_number(500, prime_vec)
}

fn first_triangle_number(max_divisors: u32, prime_vec: &Vec<u32>) -> u64 {
    let mut triangle_number = 0;
    for i in 1..u32::MAX {
        triangle_number += i;
        let num_divisors = divisors::count_divisors(triangle_number, prime_vec);
        if num_divisors > max_divisors {
            break;
        }
    }
    triangle_number as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        use crate::primes;
        assert_eq!(p12(&primes::get_primes()), 76576500);
    }

    #[test]
    fn test_first_triangle_number() {
        use crate::primes;
        assert_eq!(first_triangle_number(5, &primes::get_primes()), 28);
    }
}
