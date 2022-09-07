use std::error::Error;

pub fn p31() -> Result<u64, Box<dyn Error>> {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    count_coin_sums(200, &coins)
}

fn count_coin_sums(total: u64, coins: &Vec<u64>) -> Result<u64, Box<dyn Error>> {
    if coins.is_empty() {
        return Ok(0);
    }
    if total == 0 {
        return Ok(1);
    }

    let mut coins = coins.clone();
    coins.sort_unstable();
    let mut cache: Vec<Vec<u64>> = Vec::with_capacity(coins.len());

    let mut first_row = Vec::with_capacity((total + 1) as usize);
    let ways_for_lowest_denomination = {
        if total % coins[0] == 0 {
            1
        } else {
            return Err("Our algorithm does not currently handle coprime lowest denomination and current total. We could, but that would require fixing things.".into());
        }
    };
    for _ in 0..=total {
        first_row.push(ways_for_lowest_denomination);
    }
    cache.push(first_row);

    for coin in coins.iter().skip(1) {
        let mut new_row = Vec::with_capacity((total + 1) as usize);
        for i in 0..=total {
            let total_with_coin = {
                if *coin <= i {
                    new_row[(i - *coin) as usize]
                } else {
                    0
                }
            };
            let total_without_coin =
                cache.last().ok_or("The cache is somehow nonempty")?[i as usize];
            new_row.push(total_with_coin + total_without_coin);
        }
        cache.push(new_row);
    }
    Ok(*cache
        .last()
        .ok_or("The cache unexpectedly has no lines")?
        .last()
        .ok_or("The last cache line unexpectedly has no items")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p31().unwrap(), 73682);
    }

    #[test]
    fn test_doubled_coins() {
        let mut coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
        for coin in &mut coins {
            *coin *= 2;
        }
        let num = count_coin_sums(400, &coins);
        assert_eq!(num.unwrap(), 73682);
    }

    #[test]
    fn test_empty_coins() {
        let coins = vec![];
        assert_eq!(count_coin_sums(1, &coins).unwrap(), 0);
    }

    #[test]
    fn test_coprime_lowest_denomination() {
        let coins = vec![2, 5];
        assert!(count_coin_sums(9, &coins).is_err());
    }

    #[test]
    fn test_count_coin_sums() {
        let coins = vec![1, 5, 10];
        assert_eq!(count_coin_sums(0, &coins).unwrap(), 1);
        assert_eq!(count_coin_sums(1, &coins).unwrap(), 1);
        assert_eq!(count_coin_sums(5, &coins).unwrap(), 2);
        assert_eq!(count_coin_sums(6, &coins).unwrap(), 2);
        assert_eq!(count_coin_sums(12, &coins).unwrap(), 4);
    }
}
