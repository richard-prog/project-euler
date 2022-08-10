use std::collections::HashMap;

pub fn p32() -> u64 {
    let mut pandigital_products = HashMap::new();
    for multiplicand in 1..10_000 {
	for multiplier in 1..(10_000 / multiplicand) {
	    let product = multiplier * multiplicand;
	    if is_pandigital_triple((multiplicand, multiplier, product)) {
		pandigital_products.insert(product, ());
	    }
	}
    }
    pandigital_products.keys().sum()
}

fn is_pandigital_triple(triple: (u64, u64, u64)) -> bool {
    let mut pandigits = [false; 9];
    let mut digits: Vec<u8> = Vec::with_capacity(10);
    digits.extend(triple.0.to_string().as_bytes());
    digits.extend(triple.1.to_string().as_bytes());
    digits.extend(triple.2.to_string().as_bytes());
    if digits.len() != 9 {
	return false;
    }
    for digit in digits {
	if digit == b'0' {
	    return false;
	}
	let index: usize = (digit - b'1').into();
	if pandigits[index] {
	    return false;
	}
	pandigits[index] = true;
    }
    pandigits.iter().all(|d| *d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pandigital_triple() {
	assert_eq!(is_pandigital_triple((12, 345, 6789)), true);
	assert_eq!(is_pandigital_triple((39, 186, 7254)), true);
	assert_eq!(is_pandigital_triple((12, 240, 2880)), false);
	assert_eq!(is_pandigital_triple((0, 0, 0)), false);
    }
}
