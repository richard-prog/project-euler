use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn p26() -> u64 {
    let mut cur_max = 0;
    for i in 1..1000 {
	if i % 2 == 0 || i % 5 == 0 {
	    continue;
	}
	let cycle_length = positive_cycle_length(i);
	if cycle_length > cur_max {
	    cur_max = i;
	}
    }
    cur_max
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct DecimalDigit {
    digit: u8,
    remainder: u64,
}

fn positive_cycle_length(denominator: u64) -> u64 {
    let mut digits: HashMap<DecimalDigit, u64> = HashMap::new();
    let mut place = 1;
    let mut cur = 1;
    loop {
	cur *= 10;
	let d = DecimalDigit{
	    digit: (cur / denominator) as u8,
	    remainder: (cur % denominator) as u64
	};
	if let Entry::Occupied(digit) = digits.entry(d) {
	    return place - digit.get();
	} else {
	    digits.insert(d, place);
	    cur = d.remainder;
	    place += 1;
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p26(), 983);
    }

    #[test]
    fn test_cycle_length() {
	assert_eq!(positive_cycle_length(3), 1, "Denominator = 3");
	assert_eq!(positive_cycle_length(6), 1, "Denominator = 6");
	assert_eq!(positive_cycle_length(7), 6, "Denominator = 7");
	assert_eq!(positive_cycle_length(11), 2, "Denominator = 11");
    }
}
