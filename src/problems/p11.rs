use std::fs;

pub fn p11() -> u64 {
    let v = read_from_file();
    
    largest_product(&v, 4) as u64
}

fn read_from_file() -> Vec<Vec<u8>> {
    let s = fs::read_to_string("p11.txt").unwrap();
    let mut v = Vec::<Vec<u8>>::new();
    let mut capacity = 0;
    let mut num: u8 = 0;
    let mut new_vec = Vec::with_capacity(capacity);
    for &b in s.as_bytes() {
	if b'0' <= b && b <= b'9' {
	    num = num * 10 + b - b'0';
	} else if b == b' ' {
	    new_vec.push(num);
	    capacity += 1;
	    num = 0;
	} else if b == b'\n' {
	    new_vec.push(num);
	    v.push(new_vec);
	    new_vec = Vec::with_capacity(capacity + 1);
	    capacity = 0;
	    num = 0;
	} else {
	    panic!("File is only supposed to have digits, spaces, and newlines");
	}
    }
    v
}
    
fn largest_product(v: &Vec<Vec<u8>>, l: usize) -> u32 {
    let m = v.len();
    let n = v[0].len();
    let mut max_product = 0;
    'outer: for row in v {
	for &num in row {
	    if num != 0 {
		max_product = 1;
		break 'outer;
	    }
	}
    }
    if max_product == 0 {
	return 0;
    }
    //horizontal
    for i in 0..m {
    	for j in 0..(n - l) {
    	    let mut cur_product = 1;
    	    for k in 0..l {
    		cur_product *= v[i][j + k] as u32;
    	    }
    	    if cur_product > max_product {
    		max_product = cur_product;
    	    }
    	}
    }
    //vertical
    for j in 0..n {
    	for i in 0..(m - l) {
    	    let mut cur_product = 1;
    	    for k in 0..l {
    		cur_product *= v[i + k][j] as u32;
    	    }
    	    if cur_product > max_product {
    		max_product = cur_product;
    	    }
    	}
    }
    //diagonal
    for i in 0..(m - l) {
	for j in 0..(n - l) {
	    //left-to-right diagonal
	    let mut cur_product = 1;
	    for k in 0..l {
		cur_product *= v[i + k][j + k] as u32;
	    }
	    if cur_product > max_product {
		max_product = cur_product;
	    }
	    //right-to-left diagonal
	    cur_product = 1;
	    for k in 0..l {
		cur_product *= v[i + (l - 1) - k][j + k] as u32;
	    }
	    if cur_product > max_product {
		max_product = cur_product;
	    }
	}
    }
    max_product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution() {
	assert_eq!(p11(), 70600674);
    }
}
