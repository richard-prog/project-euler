use std::fs;

pub fn read_from_file_u8(filename: &str) -> Vec<Vec<u8>> {
    let s = fs::read_to_string(filename).unwrap();
    let mut v = Vec::<Vec<u8>>::new();
    let mut capacity = 0;
    let mut num: u8 = 0;
    let mut new_vec = Vec::new();
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

pub fn read_from_file_u32(filename: &str) -> Vec<Vec<u32>> {
    let s = fs::read_to_string(filename).unwrap();
    let mut v = Vec::<Vec<u32>>::new();
    let mut capacity = 0;
    let mut num: u32 = 0;
    let mut new_vec = Vec::new();
    for &b in s.as_bytes() {
	if b'0' <= b && b <= b'9' {
	    num = num * 10 + (b - b'0') as u32;
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
