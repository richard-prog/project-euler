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
    max_grid_product(&grid, 4)
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
    
fn max_grid_product(grid: &U8Grid, num_adjacent: usize) -> u64 {
    if zero_grid(grid) {
	return 0;
    }
    let mut cur_max = 1;
    for i in 0..NUM_ROWS {
	for j in 0..NUM_COLS {
	    update_max(grid, num_adjacent, (i, j), &mut cur_max);
	}
    }
    cur_max
}

fn update_max(grid: &U8Grid,
	      num_adjacent: usize,
	      pivot: (usize, usize),
	      max: &mut u64)
{
    let(i, j) = pivot;
    let init = grid[i][j] as u64;
    let space_right = j + num_adjacent < NUM_COLS;
    let space_left = j + 1 >= num_adjacent;
    let space_below = i + num_adjacent < NUM_ROWS;
    if space_right {
	let mut horizontal = init;
    	for offset in 1..num_adjacent {
    	    horizontal *= grid[i][j + offset] as u64;
    	}
	if horizontal > *max {
	    *max = horizontal;
	}
    }
    
    let (mut vertical, mut major, mut minor) = (init, init, init);
    if space_below {
    	for offset in 1..num_adjacent {
    	    vertical *= grid[i + offset][j] as u64;
	    
    	    if space_right {
    		major *= grid[i + offset][j + offset] as u64;
    	    }
    	    if space_left {
    		minor *= grid[i + offset][j - offset] as u64;
    	    }
    	}
    }
    if vertical > *max {
	*max = vertical;
    }
    if major > *max {
	*max = major;
    }
    if minor > *max {
	*max = minor;
    }
}

fn zero_grid(grid: &U8Grid) -> bool {
    grid.iter().take(NUM_ROWS).all(zero_row)
}

fn zero_row(row: &Row) -> bool {
    row.iter().take(NUM_COLS).all(|x| *x == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p11(), 70600674);
    }


    #[test]
    fn check_update_max() {
	let grid = read_from_file();
	let mut max = 0;
	update_max(&grid, 4, (0, 0), &mut max);
	assert_eq!(max, 1651104);
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
}
