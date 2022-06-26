use std::fs;
use crate::symbolic_math;

pub fn p13() -> u64 {
    let s = fs::read_to_string("p13.txt").unwrap();

    let mut vecs: Vec<Vec<u8>> = Vec::with_capacity(100);
    for i in 0..100 {
	vecs.push(Vec::from(&s[(51 * i)..(50 + 51 * i)]));
    }
    let mut result = Vec::with_capacity(100);
    for vec in vecs {
	result = symbolic_math::symbolic_add(&result, &vec);
    }
    std::str::from_utf8(&result).unwrap()[..10].parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p13(), 5537376230);
    }
}
