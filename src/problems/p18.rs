use crate::maximum_path_sum;
use crate::read_from_file;
use std::error::Error;

pub fn p18() -> Result<u64, Box<dyn Error>> {
    let mut v = read_from_file::read_from_file_u32("p18.txt")?;
    maximum_path_sum::maximum_path_sum(&mut v);
    Ok(v[0][0] as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
        assert_eq!(p18().unwrap(), 1074);
    }
}
