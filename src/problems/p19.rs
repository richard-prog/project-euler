pub fn p19() -> u64 {
    let mut date = Date::from(1900, January, 1);
    let mut count_full = 0;
    while date.year < 2001 {
	advance(&mut count_full, &mut date);
    }
    
    date = Date::from(1900, January, 1);
    let mut count_partial = 0;
    while date.year < 1901 {
	advance(&mut count_partial, &mut date);
    }
    count_full - count_partial
}

use Month::{January, February, March, April, May, June, July, August, September, October, November, December};

struct Date {
    year: u16,
    month: Month,
    day: u8,
}

impl Date {
    fn from(year: u16, month: Month, day: u8) -> Date {
	Date {year, month, day}
    }
}

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

fn advance(count: &mut u64, date: &mut Date) {
    if date.day == 0 {
	*count += 1;
    }

    match date.month {
	September | April | June | November => {
	    date.day = (date.day + 30) % 7;
	},
	January | March | May | July | August | October => {
	    date.day = (date.day + 31) % 7;
	},
	December => {
	    date.day = (date.day + 31) % 7;
	    date.year += 1;
	},
	February => {
	    if is_leap_year(date.year as u32) {
		date.day = (date.day + 29) % 7;
	    }
	    //No else: (*day + 28) % 7 == day
	}
    }
    date.month = date.month.next();
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
    false
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
