const GRID_SIZE: usize = 21;

pub fn p15() -> u64 {
    let mut grid: [[u64; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    for j in 1..GRID_SIZE {
    	grid[0][j] = 1;
    }

    for i in 1..GRID_SIZE {
    	grid[i][0] = 1;
    	for j in 1..GRID_SIZE {
    	    grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
    	}
    }
    grid[GRID_SIZE - 1][GRID_SIZE - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p15(), 137846528820);
    }
}
