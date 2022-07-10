use serde_json;
use std::fs;

const MAX: usize = 2_000_000;

pub fn generate_primes() -> Vec<u32> {
    let mut sieve = [true; MAX + 1];
    let max_factor = ((MAX as f64).sqrt()) as usize;
    for i in 2..=max_factor {
        if !sieve[i] {
            // i is not prime; all multiples of i already eliminated
            continue;
        }
        let mut j = 2 * i;
        while j <= MAX {
            sieve[j] = false;
            j += i;
        }
    }
    let mut prime_vec: Vec<u32> = Vec::with_capacity(MAX / (MAX as f64).log(3.0) as usize);
    for (i, p) in sieve.iter().enumerate().take(MAX + 1).skip(2) {
        if *p {
            prime_vec.push(i as u32);
        }
    }
    // let json_string = serde_json::to_string(&prime_vec).unwrap();
    // fs::write("primes.txt", json_string).unwrap();
    prime_vec
}

pub fn get_primes() -> Vec<u32> {
    let json_string = fs::read_to_string("primes.txt").unwrap();
    let prime_vec: Vec<u32> = serde_json::from_str(&json_string).unwrap();
    prime_vec
}

pub fn factor(mut num: u64, primes: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut ret = Vec::new();
    for &p in primes {
        // println!("{p}");
        if num % p as u64 == 0 {
            let mut exp = 0;
            while num % p as u64 == 0 {
                exp += 1;
                num /= p as u64;
            }
            ret.push((p, exp));
        }
        if num == 1 {
            break;
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_primes() {
        assert_eq!(generate_primes(), get_primes());
    }

    #[test]
    fn test_factor() {
        let prime_vec = get_primes();
        assert_eq!(factor(12, &prime_vec), vec![(2, 2), (3, 1)]);
    }
}
