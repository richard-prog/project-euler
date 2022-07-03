use crate::symbolic_math;

pub fn p16() -> u64 {
    symbolic_math::sum_symbolic_digits(&symbolic_two_to_the_power(1000)) as u64
}

fn symbolic_two_to_the_power(exp: u32) -> Vec<u8> {
    let mut init = vec![b'1'];
    for _ in 0..exp {
	let cur = init.clone();
	init = symbolic_math::symbolic_add(&cur, &cur);
    }
    init
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p16(), 1366);
    }

    #[test]
    fn test_symbolic_two_to_the_power() {
	let two_9 = symbolic_two_to_the_power(9);
	assert_eq!(two_9, vec![b'5', b'1', b'2']);
    }
}
