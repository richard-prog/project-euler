use crate::symbolic_math;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

const NUM_NUMBERS: usize = 100;
const NUM_DIGITS: usize = 50;
const NUMBER_FILE: &str = "p13.txt";

pub fn p13() -> Result<u64, Box<dyn Error>> {
    let numbers = get_numbers(NUMBER_FILE)?;
    let mut result = Vec::with_capacity(NUM_DIGITS);
    for num in numbers {
        result = symbolic_math::symbolic_add(&result, &num);
    }
    Ok(std::str::from_utf8(&result)?[..10].parse()?)
}

fn get_numbers(filename: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let mut numbers: Vec<Vec<u8>> = Vec::with_capacity(NUM_NUMBERS);
    let lines = BufReader::new(File::open(&filename)?).lines();
    for line in lines {
        numbers.push(Vec::from(line?.as_bytes()));
    }
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p13().unwrap(), 5537376230);
    }
}
