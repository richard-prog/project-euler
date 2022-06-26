const NUM_TERMS: usize = 1_000_000;

pub fn p14() -> u64 {
    let mut cache = Vec::with_capacity(NUM_TERMS);
    cache.push(1);
    let mut longest_sequence = 1;
    let mut longest_collatz = 1;
    let mut collatz: u32 = 2;
    while collatz as usize <= NUM_TERMS {
	let mut count = 0;
	let mut cur_sequence: u64 = collatz as u64;
	while cur_sequence >= collatz as u64 {
	    cur_sequence = next(cur_sequence as u64);
	    count += 1;
	}
	//cur_sequence < collatz, result cached
	//invariant: after every loop, cache[0..=collatz-1] full
	let total_count = count + cache[cur_sequence as usize - 1];
	if total_count > longest_sequence {
	    longest_sequence = total_count;
	    longest_collatz = collatz;
	}
	//cache[collatz as usize - 1] = count + cache[cur_sequence as usize - 1];
	cache.push(total_count);
	collatz += 1;
    }
    longest_collatz as u64
}

fn next(n: u64) -> u64 {
    if n % 2 == 0 {
	return n / 2;
    }
    3 * n + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p14(), 837799);
    }
}
