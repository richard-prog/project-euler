use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_from_file_u8(filename: &str) -> Vec<Vec<u8>> {
    let lines = BufReader::new(File::open(filename).unwrap()).lines();
    let mut v = Vec::<Vec<u8>>::new();
    for line in lines {
        let mut new_vec = match v.last() {
            None => Vec::new(),
            Some(prev_vec) => Vec::with_capacity(prev_vec.len() + 1),
        };
        for num in line.unwrap().split(' ') {
            new_vec.push(num.parse::<u8>().unwrap());
        }
        v.push(new_vec);
    }
    v
}

pub fn read_from_file_u32(filename: &str) -> Vec<Vec<u32>> {
    let lines = BufReader::new(File::open(filename).unwrap()).lines();
    let mut v = Vec::<Vec<u32>>::new();
    for line in lines {
        let mut new_vec = match v.last() {
            None => Vec::new(),
            Some(prev_vec) => Vec::with_capacity(prev_vec.len() + 1),
        };
        for num in line.unwrap().split(' ') {
            new_vec.push(num.parse::<u32>().unwrap());
        }
        v.push(new_vec);
    }
    v
}
