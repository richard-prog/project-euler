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

#[derive(PartialEq, Eq, Debug)]
struct Date {
    year: u16,
    month: Month,
    day_of_week: u8,
}

impl Date {
    fn from(year: u16, month: Month, day_of_week: u8) -> Date {
	Date {year, month, day_of_week}
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
    if date.day_of_week == 0 {
	*count += 1;
    }

    match date.month {
	September | April | June | November => {
	    date.day_of_week = (date.day_of_week + 30) % 7;
	},
	January | March | May | July | August | October => {
	    date.day_of_week = (date.day_of_week + 31) % 7;
	},
	December => {
	    date.day_of_week = (date.day_of_week + 31) % 7;
	    date.year += 1;
	},
	February => {
	    if is_leap_year(date.year as u32) {
		date.day_of_week = (date.day_of_week + 29) % 7;
	    }
	    //No else: (*day_of_week + 28) % 7 == day_of_week
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

    #[test]
    fn test_advance() {
	let mut count = 0;
	let mut date = Date::from(2022, June, 3);
	advance(&mut count, &mut date);
	assert_eq!(count, 0);
	assert_eq!(date, Date::from(2022, July, 5));
	advance(&mut count, &mut date);
	assert_eq!(count, 0);
	assert_eq!(date, Date::from(2022, August, 1));
    }

    #[test]
    fn test_advance_leap_year() {
	let mut count = 0;
	let mut date = Date::from(2019, November, 5);
	advance(&mut count, &mut date);
	assert_eq!(count, 0);
	assert_eq!(date, Date::from(2019, December, 0));
	advance(&mut count, &mut date);
	assert_eq!(count, 1);
	assert_eq!(date, Date::from(2020, January, 3));
	advance(&mut count, &mut date);
	assert_eq!(count, 1);
	assert_eq!(date, Date::from(2020, February, 6));
	advance(&mut count, &mut date);
	assert_eq!(count, 1);
	assert_eq!(date, Date::from(2020, March, 0));
	advance(&mut count, &mut date);
	assert_eq!(count, 2);
	assert_eq!(date, Date::from(2020, April, 3));
    }
}
