use std::cmp;

pub fn maximum_path_sum(triangle: &mut Vec<Vec<u32>>) {
    let mut i = triangle.len() - 1;
    while i > 0 {
        i -= 1;
        for j in 0..(i + 1) {
            let below = triangle[i + 1][j];
            let below_right = triangle[i + 1][j + 1];
            triangle[i][j] += cmp::max(below, below_right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_path_sum() {
        let mut triangle = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        let result = vec![vec![23], vec![20, 19], vec![10, 13, 15], vec![8, 5, 9, 3]];
        maximum_path_sum(&mut triangle);
        assert_eq!(triangle, result);
    }
}
