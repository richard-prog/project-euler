pub fn p24() -> u64 {
    let v = vec![
	String::from("0"),
	String::from("1"),
	String::from("2"),
	String::from("3"),
	String::from("4"),
	String::from("5"),
	String::from("6"),
	String::from("7"),
	String::from("8"),
	String::from("9"),
    ];
    let result_string = &permutations(&v)[999_999];
    result_string.parse().unwrap()
}

fn permutations(v: &Vec<String>) -> Vec<String> {
    if v.len() < 2 {
	return v.clone();
    }
    let mut result = Vec::new();
    for (i, s) in v.iter().enumerate() {
	let mut clone: Vec<String> = v.clone();
	clone.remove(i);
	let subpermutations = permutations(&clone);
	for permutation in subpermutations {
	    result.push(format!("{s}{permutation}"));
	}
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p24(), 2783915460);
    }

    #[test]
    fn test_permutations() {
	let v = vec![String::from("0"),
		     String::from("1"),
		     String::from("2")];
	assert_eq!(permutations(&v),
		   vec!["012", "021", "102", "120", "201", "210"]);
    }
}
