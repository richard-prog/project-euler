pub fn p19() -> u64 {
    let mut year = 1900;
    let mut day = 1;
    let mut month = January;
    let mut count_full = 0;
    while year < 2001 {
	advance(&mut count_full, &mut year, &mut month, &mut day);
    }
    let mut year = 1900;
    let mut day = 1;
    let mut month = January;
    let mut count_partial = 0;
    while year < 1901 {
	advance(&mut count_partial, &mut year, &mut month, &mut day);
    }
    count_full - count_partial
}

use Month::{January, February, March, April, May, June, July, August, September, October, November, December};

#[derive(Copy, Clone)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn next(&self) -> Month {
	match self {
	    January => February,
	    February => March,
	    March => April,
	    April => May,
	    May => June,
	    June => July,
	    July => August,
	    August => September,
	    September => October,
	    October => November,
	    November => December,
	    December => January,
	}
    }
}

fn advance(count: &mut u64, year: &mut u32, month: &mut Month, day: &mut u8) {
    if *day == 0 {
	*count += 1;
    }

    match month {
	September | April | June | November => {
	    *day = (*day + 30) % 7;
	},
	January | March | May | July | August | October => {
	    *day = (*day + 31) % 7;
	},
	December => {
	    *day = (*day + 31) % 7;
	    *year += 1;
	},
	February => {
	    if is_leap_year(*year) {
		*day = (*day + 29) % 7;
	    }
	    //No else: (*day + 28) % 7 == day
	}
    }
    *month = month.next();
}

fn is_leap_year(year: u32) -> bool {
    if year % 4 == 0 {
	if year % 100 == 0 {
	    if year % 400 == 0 {
		return true;
	    }
	    return false;
	}
	return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p19(), 171);
    }

    #[test]
    fn test_is_leap_year() {
	assert_eq!(is_leap_year(1), false);
	assert_eq!(is_leap_year(1996), true);
	assert_eq!(is_leap_year(1900), false);
	assert_eq!(is_leap_year(2000), true);
    }
}
