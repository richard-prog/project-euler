const NUM_ROWS: usize = 20;
const NUM_COLS: usize = 20;

type Row = [u8; NUM_COLS];
type U8Grid = [Row; NUM_ROWS];

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn p11() -> u64 {
    if NUM_ROWS == 0 || NUM_COLS == 0 {
	return 0;
    }
    let grid = read_from_file();
    
    largest_product(&grid, 4) as u64
}

fn read_from_file() -> U8Grid {
    let lines = BufReader::new(File::open("p11.txt").unwrap()).lines();
    let mut grid: U8Grid = [[0; NUM_COLS]; NUM_ROWS];
    for (i, line) in lines.enumerate() {
	for (j, num) in line.unwrap().split(' ').enumerate(){
	    grid[i][j] = num.parse::<u8>().unwrap();
	}
    }
    grid
}
    
fn largest_product(grid: &U8Grid, num_adjacent: usize) -> u64 {
    if zero_grid(grid) {
	return 0;
    }
    let products = vec![
	largest_horizontal_product(grid, num_adjacent),
	largest_vertical_product(grid, num_adjacent),
	largest_major_diagonal_product(grid, num_adjacent),
	largest_minor_diagonal_product(grid, num_adjacent),
    ];
    *products.iter().max().unwrap()
}

fn zero_row(row: &Row) -> bool {
    row.iter().take(NUM_COLS).all(|x| *x == 0)
}

fn zero_grid(grid: &U8Grid) -> bool {
    grid.iter().take(NUM_ROWS).all(zero_row)
}

fn largest_horizontal_product(grid: &U8Grid, num_adjacent: usize) -> u64 {
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

fn largest_vertical_product(grid: &U8Grid, num_adjacent: usize) -> u64 {
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

fn largest_major_diagonal_product(grid: &U8Grid, num_adjacent: usize) -> u64 {
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

fn largest_minor_diagonal_product(grid: &U8Grid, num_adjacent: usize) -> u64 {
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

    #[test]
    fn check_zero_row() {
	assert_eq!(zero_row(&[0; 20]), true);
	assert_eq!(zero_row(&[1; 20]), false);
    }

    #[test]
    fn check_zero_grid() {
	assert_eq!(zero_grid(&[[0; 20]; 20]), true);
	assert_eq!(zero_grid(&[[1; 20]; 20]), false);
    }

    #[test]
    fn check_largest_products() {
	let grid = read_from_file();
	assert_eq!(largest_horizontal_product(&grid, 4), 48477312);
	assert_eq!(largest_vertical_product(&grid, 4), 51267216);
	assert_eq!(largest_major_diagonal_product(&grid, 4), 32719995);
	assert_eq!(largest_minor_diagonal_product(&grid, 4), 70600674);
    }
}
