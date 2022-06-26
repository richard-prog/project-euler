use crate::symbolic_math;

pub fn p16() -> u64 {
    sum_digits(symbolic_exponentiate(1000)) as u64
}

fn symbolic_exponentiate(exp: u32) -> Vec<u8> {
    let mut init = vec![b'1'];
    for _i in 0..exp {
	let cur = init.clone();
	init = symbolic_math::symbolic_add(&cur, &cur);
    }
    init
}

fn sum_digits(digits: Vec<u8>) -> u32 {
    let mut sum: u32 = 0;
    for digit in digits {
	sum += digit as u32 - b'0' as u32;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p16(), 1366);
    }

    #[test]
    fn test_symbolic_exponentiate() {
	let two_9 = symbolic_exponentiate(9);
	assert_eq!(two_9, vec![b'5', b'1', b'2']);
    }

    #[test]
    fn test_add_digits() {
	let num = vec![b'5', b'2', b'4'];
	assert_eq!(sum_digits(num), 11)
    }
}
