use std::fs;

pub fn p22() -> u64 {
    let mut v: Vec<String> = fs::read_to_string("p022_names.txt")
    	.unwrap()
    	.split(',')
    	.map(|s| -> String {
	    s.trim_matches('"').to_string()
	})
	.collect();
    v.sort();
    v.iter()
	.enumerate()
	.fold(0, |cur, (i, val)| -> u64 {
    	    cur + (i as u64 + 1) * name_score(val)}
    	)
}

fn name_score(name: &str) -> u64 {
    name.as_bytes()
	.iter()
	.fold(0, |cur, c| -> u64 {
	    cur + (c - b'A' + 1) as u64
	})
}
	
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p22(), 871198282);
    }

    #[test]
    fn test_name_score() {
	assert_eq!(name_score("COLIN"), 53);
    }
}
