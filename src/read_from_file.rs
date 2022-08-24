use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

pub fn read_from_file_u32(filename: &str) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let lines = BufReader::new(File::open(filename)?).lines();
    let mut v = Vec::<Vec<u32>>::new();
    for line in lines {
        let mut new_vec = match v.last() {
            None => Vec::new(),
            Some(prev_vec) => Vec::with_capacity(prev_vec.len() + 1),
        };
        for num in line?.split(' ') {
            new_vec.push(num.parse::<u32>()?);
        }
        v.push(new_vec);
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from_file_u32() {
	let mut zero_grid: Vec<Vec<u32>> = Vec::with_capacity(20);
	for _ in 0..20 {
	    let mut new_vec: Vec<u32> = Vec::with_capacity(20);
	    for _ in 0..20 {
		new_vec.push(0);
	    }
	    zero_grid.push(new_vec);
	}
	let read_vec = read_from_file_u32("p11-test-zero-grid.txt");
	assert_eq!(zero_grid, read_vec.unwrap());
    }
}
