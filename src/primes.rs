use std::fs::File;
use std::io::Read;

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
    // let mut f = File::create("primes.txt").unwrap();
    // f.write(as_u8_slice(&prime_vec)).unwrap();
    (sieve.to_vec(), prime_vec)
}

pub fn get_primes() -> Vec<u32> {
    // let json_string = fs::read_to_string("primes.txt").unwrap();
    // let prime_vec: Vec<u32> = serde_json::from_str(&json_string).unwrap();
    let mut file = File::open("primes.txt").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let prime_vec = from_u8_vec(buffer);
    for i in 0..100 {
	println!("{}", prime_vec[i]);
    }
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

// fn as_u8_slice(v: &[u32]) -> &[u8] {
//     unsafe {
// 	let u8_ptr = v.as_ptr() as *const u8;
// 	let num_bytes = v.len() * std::mem::size_of::<u32>();
// 	std::slice::from_raw_parts(u8_ptr, num_bytes)
//     }
// }

fn from_u8_vec(v: Vec<u8>) -> Vec<u32> {
    let ptr = v.as_ptr();
    let length = v.len();
    let capacity = v.capacity();
    let element_size = std::mem::size_of::<u32>();

    assert_eq!(capacity % element_size, 0);

    unsafe {
	std::mem::forget(v);

	Vec::from_raw_parts(
	    ptr as *mut u32,
	    length / element_size,
	    capacity / element_size
	)
    }
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
