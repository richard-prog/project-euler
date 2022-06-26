use crate::read_from_file;
use crate::maximum_path_sum;

pub fn p18() -> u64 {
    let mut v = read_from_file::read_from_file_u32("p18.txt");
    maximum_path_sum::maximum_path_sum(&mut v);
    v[0][0] as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p18(), 1074);
    }
}
