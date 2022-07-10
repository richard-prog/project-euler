use std::fs;

pub fn p22() -> u64 {
    let mut names: Vec<String> = fs::read_to_string("p022_names.txt")
        .unwrap()
        .split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect();
    names.sort();
    names
        .iter()
        .enumerate()
        .fold(0, |cur, (i, val)| cur + (i as u64 + 1) * name_score(val))
}

fn name_score(name: &str) -> u64 {
    name.as_bytes()
        .iter()
        .fold(0, |cur, letter| cur + (letter - b'A' + 1) as u64)
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
