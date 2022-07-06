pub fn p31() -> u64 {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    count_coin_sums(200, &coins)
} 

fn count_coin_sums(total: u64, coins: &Vec<u64>) -> u64{
    if coins.is_empty() {
	return 0;
    }
    if total == 0 {
	return 1;
    }
    
    let mut coins = coins.clone();
    coins.sort_unstable();
    let mut cache: Vec<Vec<u64>> = Vec::with_capacity(coins.len());
    
    let mut first_row = Vec::with_capacity((total+1) as usize);
    let ways_for_lowest_denomination = {
	if total % coins[0] == 0 {
	    1
	} else {
	    0
	}
    };
    for _ in 0..=total {
	first_row.push(ways_for_lowest_denomination);
    }
    cache.push(first_row);
    
    for coin in coins.iter().skip(1) {
	let mut new_row = Vec::with_capacity((total+1) as usize);
	for i in 0..=total {
	    let total_with_coin = {
		if *coin <= i {
		    new_row[(i - *coin) as usize]
		} else {
		    0
		}
	    };
	    let total_without_coin = cache.last().unwrap()[i as usize];
	    new_row.push(total_with_coin + total_without_coin);
	}
	cache.push(new_row);
    }
    *cache.last().unwrap().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
    	assert_eq!(p31(), 73682);
    }

    #[test]
    fn test_count_coin_sums() {
	let coins = vec![1, 5, 10];
	assert_eq!(count_coin_sums(1, &coins), 1);
	assert_eq!(count_coin_sums(5, &coins), 2);
	assert_eq!(count_coin_sums(6, &coins), 2);
	assert_eq!(count_coin_sums(12, &coins), 4);
    }
}
