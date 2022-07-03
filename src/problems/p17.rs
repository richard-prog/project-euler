pub fn p17() -> u64 {
    let mut count = 0;
    for i in 1..=1000 {
	count += count_letters(&num_to_words(i));
    }
    count as u64
}

fn num_to_words(num: u32) -> String {
    match num {
	1 => "one".to_string(),
	2 => "two".to_string(),
	3 => "three".to_string(),
	4 => "four".to_string(),
	5 => "five".to_string(),
	6 => "six".to_string(),
	7 => "seven".to_string(),
	8 => "eight".to_string(),
	9 => "nine".to_string(),
	10 => "ten".to_string(),
	11 => "eleven".to_string(),
	12 => "twelve".to_string(),
	13 => "thirteen".to_string(),
	14 => "fourteen".to_string(),
	15 => "fifteen".to_string(),
	16 => "sixteen".to_string(),
	17 => "seventeen".to_string(),
	18 => "eighteen".to_string(),
	19 => "nineteen".to_string(),
	21..=29 => format!("twenty-{}", num_to_words(num % 10)),
	31..=39 => format!("thirty-{}", num_to_words(num % 10)),
	41..=49 => format!("forty-{}", num_to_words(num % 10)),
	51..=59 => format!("fifty-{}", num_to_words(num % 10)),
	61..=69 => format!("sixty-{}", num_to_words(num % 10)),
	71..=79 => format!("seventy-{}", num_to_words(num % 10)),
	81..=89 => format!("eighty-{}", num_to_words(num % 10)),
	91..=99 => format!("ninety-{}", num_to_words(num % 10)),
	20 => "twenty".to_string(),
	30 => "thirty".to_string(),
	40 => "forty".to_string(),
	50 => "fifty".to_string(),
	60 => "sixty".to_string(),
	70 => "seventy".to_string(),
	80 => "eighty".to_string(),
	90 => "ninety".to_string(),
	100..=900 if num % 100 == 0 => format!("{} hundred", num_to_words(num / 100)),
	101..=999 => format!("{} hundred and {}", num_to_words(num / 100), num_to_words(num % 100)),
	1000 => "one thousand".to_string(),
	_ => panic!("You done goofed"),
    }
}

fn count_letters(word: &str) -> u32 {
    let mut count = 0;
    for letter in word.bytes() {
	if (b'a'..=b'z').contains(&letter) {
	    count += 1;
	}
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p17(), 21124);
    }
    
    #[test]
    fn test_num_to_words() {
	assert_eq!("forty-two", num_to_words(42));
	assert_eq!("thirty", num_to_words(30));
	assert_eq!("three hundred and forty-two", num_to_words(342));
	assert_eq!("one hundred and fifteen", num_to_words(115));
	assert_eq!("three hundred", num_to_words(300));
    }

    #[test]
    fn test_count_letters() {
	assert_eq!(count_letters(&num_to_words(115)), 20);
	assert_eq!(count_letters(&num_to_words(342)), 23);
	let mut count = 0;
	for i in 1..=5 {
	    count += count_letters(&num_to_words(i));
	}
	assert_eq!(count, 19);
    }
}
