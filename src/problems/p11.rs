const NUM_ROWS: usize = 20;
const NUM_COLS: usize = 20;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn p11() -> u64 {
    let grid = Grid::new("p11.txt", 4);
    grid.into_iter().max().unwrap()
}

struct Grid {
    grid: [[u8; NUM_COLS]; NUM_ROWS],
    i: usize,
    j: usize,
    num_adjacent: usize
}

impl Grid {
    fn new(filename: &str, num_adjacent: usize) -> Grid {
	let lines = BufReader::new(File::open(filename).unwrap()).lines();
	let mut grid = [[0; NUM_COLS]; NUM_ROWS];
	for (i, line) in lines.enumerate() {
	    for (j, num) in line.unwrap().split(' ').enumerate(){
		grid[i][j] = num.parse::<u8>().unwrap();
	    }
	}
	let i = 0;
	let j = 0;
	Grid {grid, i, j, num_adjacent}
    }

    fn max_product(&self) -> u64 {
	let(i, j) = (self.i, self.j);
	let grid = self.grid;
	let init = grid[i][j] as u64;
	if init == 0 {
	    return 0;
	}
	let mut max = 1;

	let num_adjacent = self.num_adjacent;
	let space_right = j + num_adjacent < NUM_COLS;
	let space_left = j + 1 >= num_adjacent;
	let space_below = i + num_adjacent < NUM_ROWS;
	
	if space_right {
	    let mut horizontal = init;
    	    for offset in 1..num_adjacent {
    		horizontal *= grid[i][j + offset] as u64;
    	    }
	    if horizontal > max {
		max = horizontal;
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
	if vertical > max {
	    max = vertical;
	}
	if major > max {
	    max = major;
	}
	if minor > max {
	    max = minor;
	}
	max
    }
}

impl Iterator for Grid {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
	if self.i == NUM_ROWS {
	    return None;
	}
	let product = self.max_product();
	self.j += 1;
	if self.j == NUM_COLS {
	    self.j = 0;
	    self.i += 1;
	}
	Some(product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p11(), 70600674);
    }


    #[test]
    fn test_max_product() {
    	let grid = Grid::new("p11.txt", 4);
    	assert_eq!(grid.max_product(), 1651104);
    }
}
