const GRID_SIZE: usize = 21;

pub fn p15() -> u64 {
    let mut grid: Vec<Vec<u64>> = Vec::with_capacity(GRID_SIZE);
    grid.push(Vec::with_capacity(GRID_SIZE));
    grid[0].push(0);
    for _i in 1..GRID_SIZE {
	grid[0].push(1);
    }
    for i in 1..GRID_SIZE {
	grid.push(Vec::with_capacity(GRID_SIZE));
	grid[i].push(1);
	for j in 1..GRID_SIZE {
	    let left = grid[i][j - 1];
	    let up = grid[i - 1][j];
	    grid[i].push(left + up);
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
