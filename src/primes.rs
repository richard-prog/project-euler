use serde_json;
use std::fs;

const MAX: usize = 2_000_000;

pub fn generate_primes() {
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
    for i in 2..=MAX {
        if sieve[i] {
            prime_vec.push(i as u32);
        }
    }

    let json_string = serde_json::to_string(&prime_vec).unwrap();
    //println!("{}", json_string);
    fs::write("primes.txt", json_string).unwrap();
}

pub fn get_primes() -> Vec<u32> {
    let json_string = fs::read_to_string("primes.txt").unwrap();
    let prime_vec: Vec<u32> = serde_json::from_str(&json_string).unwrap();
    prime_vec
}
