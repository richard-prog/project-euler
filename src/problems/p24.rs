pub fn p24() -> u64 {
    let digits = vec![
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
    permutation_n(&digits, 1_000_000)
}

fn permutation_n(digits: &[String], n: usize) -> u64 {
    let mut digits = digits.to_owned();
    let mut factorial: usize = (1..digits.len()).product();
    let mut target = n - 1;
    let mut result_string = String::with_capacity(digits.len());
    while target > 0 {
        result_string.push_str(&digits.remove(target / factorial));
        target %= factorial;
        factorial /= digits.len();
    }
    while !digits.is_empty() {
        result_string.push_str(&digits.remove(0));
    }
    result_string.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p24(), 2783915460);
    }

    #[test]
    fn test_permutation_n() {
        let v = vec![String::from("0"), String::from("1"), String::from("2")];
        let permutation_vec = vec!["012", "021", "102", "120", "201", "210"];
        for i in 0..permutation_vec.len() {
            assert_eq!(
                permutation_n(&v, i + 1),
                permutation_vec[i].parse::<u64>().unwrap()
            );
        }
    }
}
