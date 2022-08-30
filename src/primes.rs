use std::fs::File;
#[allow(unused_imports)]
use std::io::{Read, Write};

const MAX: usize = 2_000_000;

pub fn generate_primes() -> (Vec<bool>, Vec<u32>) {
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
    // let mut f = File::create("primes.dump").unwrap();
    // for prime in &prime_vec {
    // 	f.write(&prime.to_le_bytes()).unwrap();
    // }
    (sieve.to_vec(), prime_vec)
}

pub fn get_primes() -> Vec<u32> {
    let mut f = File::open("primes.dump").unwrap();
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).unwrap();

    let mut prime_vec = Vec::with_capacity(MAX / (MAX as f64).log(3.0) as usize);
    let mut buffer = [0; 4];
    let mut i = 0;
    let l = bytes.len();
    while i < l {
        buffer[..4].copy_from_slice(&bytes[i..(i + 4)]);
        prime_vec.push(u32::from_le_bytes(buffer));
        i += 4;
    }
    prime_vec
}

pub fn factor(mut num: u64, primes: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut ret = Vec::new();
    for &p in primes {
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
        assert_eq!(generate_primes().1, get_primes());
    }

    #[test]
    fn test_factor() {
        let prime_vec = get_primes();
        assert_eq!(factor(12, &prime_vec), vec![(2, 2), (3, 1)]);
    }
}
