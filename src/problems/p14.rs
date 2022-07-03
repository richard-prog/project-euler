const NUM_TERMS: usize = 1_000_000;

pub fn p14() -> u64 {
    let mut cache_lengths = Vec::with_capacity(NUM_TERMS);
    cache_lengths.push(1);
    let mut longest_sequence = 1;
    let mut longest_term = 1;
    let mut term: u32 = 2;
    while term as usize <= NUM_TERMS {
	let mut uncached_count = 0;
	let mut uncached_term: u64 = term as u64;
	while uncached_term >= term as u64 {
	    uncached_term = next(uncached_term as u64);
	    uncached_count += 1;
	}
	//invariant: after every loop, cache_lengths[0..=term-1] full
	let sequence_length = uncached_count + cache_lengths[uncached_term as usize - 1];
	if sequence_length > longest_sequence {
	    longest_sequence = sequence_length;
	    longest_term = term;
	}
	cache_lengths.push(sequence_length);
	term += 1;
    }
    longest_term as u64
}

fn next(term: u64) -> u64 {
    if term % 2 == 0 {
	return term / 2;
    }
    3 * term + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p14(), 837799);
    }
}
