const NUM_ROWS: usize = 20;
const NUM_COLS: usize = 20;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn p11() -> u64 {
    let grid = read_from_file();
    
    largest_product(&grid, 4) as u64
}

fn read_from_file() -> [[u8; NUM_COLS]; NUM_ROWS] {
    let lines = BufReader::new(File::open("p11.txt").unwrap()).lines();
    let mut grid: [[u8; NUM_COLS]; NUM_ROWS] = [[0; NUM_COLS]; NUM_ROWS];
    for (i, line) in lines.enumerate() {
	for (j, num) in line.unwrap().split(' ').enumerate(){
	    grid[i][j] = num.parse::<u8>().unwrap();
	}
    }
    grid
}
    
fn largest_product(grid: &[[u8; NUM_COLS]; NUM_ROWS], l: usize) -> u64 {
    let mut max_product = 0;
    'outer: for i in 0..NUM_ROWS {
	for j in 0..NUM_COLS {
	    if grid[i][j] != 0 {
		max_product = 1;
		break 'outer;
	    }
	}
    }
    if max_product == 0 {
	return 0;
    }
    //horizontal
    for i in 0..NUM_ROWS {
    	for j in 0..(NUM_COLS - l) {
    	    let mut cur_product = 1;
    	    for k in 0..l {
    		cur_product *= grid[i][j + k] as u64;
    	    }
    	    if cur_product > max_product {
    		max_product = cur_product;
    	    }
    	}
    }
    //vertical
    for j in 0..NUM_COLS {
    	for i in 0..(NUM_ROWS - l) {
    	    let mut cur_product = 1;
    	    for k in 0..l {
    		cur_product *= grid[i + k][j] as u64;
    	    }
    	    if cur_product > max_product {
    		max_product = cur_product;
    	    }
    	}
    }
    //diagonal
    for i in 0..(NUM_ROWS - l) {
	for j in 0..(NUM_COLS - l) {
	    //left-to-right diagonal
	    let mut cur_product = 1;
	    for k in 0..l {
		cur_product *= grid[i + k][j + k] as u64;
	    }
	    if cur_product > max_product {
		max_product = cur_product;
	    }
	    //right-to-left diagonal
	    cur_product = 1;
	    for k in 0..l {
		cur_product *= grid[i + (l - 1) - k][j + k] as u64;
	    }
	    if cur_product > max_product {
		max_product = cur_product;
	    }
	}
    }
    max_product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p11(), 70600674);
    }
}
