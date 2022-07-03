use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::symbolic_math;

const NUM_NUMBERS: usize = 100;
const NUM_DIGITS: usize = 50;
const NUMBER_FILE: &str = "p13.txt";

pub fn p13() -> u64 {
    let numbers = get_numbers(NUMBER_FILE);
    let mut result = Vec::with_capacity(NUM_DIGITS);
    for num in numbers {
	result = symbolic_math::symbolic_add(&result, &num);
    }
    std::str::from_utf8(&result).unwrap()[..10].parse().unwrap()
}

fn get_numbers(filename: &str) -> Vec<Vec<u8>>{
    let mut numbers: Vec<Vec<u8>> = Vec::with_capacity(NUM_NUMBERS);
    let lines = BufReader::new(File::open(&filename).unwrap()).lines();
    for line in lines {
	numbers.push(Vec::from(line.unwrap().as_bytes()));
    }
    numbers
}
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p13(), 5537376230);
    }
}
