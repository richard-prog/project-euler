const NUM_ROWS: usize = 20;
const NUM_COLS: usize = 20;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn p11() -> u64 {
    if NUM_ROWS == 0 || NUM_COLS == 0 {
	return 0;
    }
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
    
fn largest_product(grid: &[[u8; NUM_COLS]; NUM_ROWS], num_adjacent: usize) -> u64 {
    let mut max_product = 0;
    'outer: for row in grid.iter().take(NUM_ROWS) {
	for element in row.iter().take(NUM_COLS) {
	    if *element != 0 {
		max_product = 1;
		break 'outer;
	    }
	}
    }
    if max_product == 0 {
	return 0;
    }
    let mut products = Vec::with_capacity(4);
    products.push(largest_horizontal_product(&grid, num_adjacent));
    products.push(largest_vertical_product(&grid, num_adjacent));
    products.push(largest_major_diagonal_product(&grid, num_adjacent));
    products.push(largest_minor_diagonal_product(&grid, num_adjacent));
    *products.iter().max().unwrap()
}

fn largest_horizontal_product(grid: &[[u8; NUM_COLS]; NUM_ROWS], num_adjacent: usize) -> u64 {
    let mut max_product = 1;
    for row in grid.iter().take(NUM_ROWS) {
	for j in 0..(NUM_COLS - num_adjacent) {
	    let mut cur_product = 1;
	    for k in 0..num_adjacent {
		cur_product *= row[j + k] as u64;
	    }
	    if cur_product > max_product {
		max_product = cur_product;
	    }
	}
    }
    max_product
}

fn largest_vertical_product(grid: &[[u8; NUM_COLS]; NUM_ROWS], num_adjacent: usize) -> u64 {
    let mut max_product = 1;
    for j in 0..NUM_COLS {
    	for i in 0..(NUM_ROWS - num_adjacent) {
    	    let mut cur_product = 1;
    	    for k in 0..num_adjacent {
    		cur_product *= grid[i + k][j] as u64;
    	    }
    	    if cur_product > max_product {
    		max_product = cur_product;
    	    }
    	}
    }
    max_product
}

fn largest_major_diagonal_product(grid: &[[u8; NUM_COLS]; NUM_ROWS], num_adjacent: usize) -> u64 {
    let mut max_product = 1;
    for i in 0..(NUM_ROWS - num_adjacent) {
	for j in 0..(NUM_COLS - num_adjacent) {
	    let mut cur_product = 1;
	    for k in 0..num_adjacent {
		cur_product *= grid[i + k][j + k] as u64;
	    }
	    if cur_product > max_product {
		max_product = cur_product;
	    }
	}
    }
    max_product
}

fn largest_minor_diagonal_product(grid: &[[u8; NUM_COLS]; NUM_ROWS], num_adjacent: usize) -> u64 {
    let mut max_product = 1;
    for i in 0..(NUM_ROWS - num_adjacent) {
	for j in 0..(NUM_COLS - num_adjacent) {
	    let mut cur_product = 1;
	    for k in 0..num_adjacent {
		let row = i + (num_adjacent - 1) - k;
		cur_product *= grid[row][j + k] as u64;
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
