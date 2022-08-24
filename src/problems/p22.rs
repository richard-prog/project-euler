use std::error::Error;
use std::fs;

pub fn p22() -> Result<u64, Box<dyn Error>> {
    let mut names: Vec<String> = fs::read_to_string("p022_names.txt")?
        .split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect();
    names.sort();
    let total_name_score = names
        .iter()
        .enumerate()
        .fold(0, |cur, (i, val)| cur + (i as u64 + 1) * name_score(val));
    Ok(total_name_score)
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
        assert_eq!(p22().unwrap(), 871198282);
    }

    #[test]
    fn test_name_score() {
        assert_eq!(name_score("COLIN"), 53);
    }
}
