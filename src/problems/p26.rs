use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn p26() -> u64 {
    let mut cur_max = 0;
    for i in 1..1000 {
	let l = cycle_length(i);
	if l > cur_max {
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

fn zero_cycle_length(denominator: u64) -> bool {
    let mut n = denominator;
    while n % 2 == 0 {
	n /= 2;
    }
    while n % 5 == 0 {
	n /= 5;
    }
    n == 1
}

fn cycle_length(denominator: u64) -> u64 {
    if zero_cycle_length(denominator) {
	return 0;
    }
    let mut digits: HashMap<DecimalDigit, u64> = HashMap::new();
    let mut place = 1;
    let mut cur = 1;
    loop {
	cur *= 10;
	let d = DecimalDigit{digit: (cur / denominator) as u8, remainder: (cur % denominator) as u64};
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
    fn zzz() {
	assert_eq!(p26(), 983);
    }

    #[test]
    fn test_cycle_length() {
	assert_eq!(cycle_length(2), 0, "Denominator = 2");
	assert_eq!(cycle_length(3), 1, "Denominator = 3");
	assert_eq!(cycle_length(4), 0, "Denominator = 4");
	assert_eq!(cycle_length(6), 1, "Denominator = 6");
	assert_eq!(cycle_length(7), 6, "Denominator = 7");
	assert_eq!(cycle_length(11), 2, "Denominator = 11");
    }
}
