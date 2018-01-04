use common;
use std::str::FromStr;

pub struct KnotHasher {
    vec: Vec<u8>,
    pos: usize,
    skip: usize,
}

impl KnotHasher {
    pub fn hash(input: &str) -> String {
        let mut lens: Vec<u8> = input.trim().as_bytes().into();
        lens.extend([17u8, 31u8, 73u8, 47u8, 23u8].iter());

        let mut hasher = KnotHasher::new(gen_list());
        for _ in 0..64 {
            hasher.run(&lens);
        }
        let sparse = hasher.to_slice();
        let dense: Vec<String> = sparse.chunks(16).map(|slice| {
            let byte = slice[1..].iter().fold(slice[0], |acc, n| acc ^ n);
            format!("{:02x}", byte)
        }).collect();
        dense.join("")
    }

    fn new(vec: Vec<u8>) -> Self {
        KnotHasher { vec, pos: 0, skip: 0 }
    }

    fn run(&mut self, lengths: &[u8]) {
        for len in lengths {
            self.process_length(*len as usize);
        }
    }

    fn process_length(&mut self, len: usize) {
        for i in 0 .. len/2 {
            let start = self.get_wrapping_index(self.pos + i);
            let end = self.get_wrapping_index(self.pos + len - 1 - i);
            self.vec.swap(start, end);
        }

        self.pos = self.get_wrapping_index(self.pos + len + self.skip);
        self.skip += 1;
    }

    fn get_wrapping_index(&self, idx: usize) -> usize {
        let mut result = idx;
        while result >= self.vec.len() {
            result -= self.vec.len()
        };

        result
    }

    fn to_slice(&self) -> &[u8] {
        &self.vec
    }
}

pub fn run(_args: &[String]) {
    let input = common::get_input("./inputs/10.txt").expect("expected input 10.txt");

    { // Part 1
        let lens: Vec<u8> = input.split(',').map(|s| u8::from_str(&s).unwrap()).collect();
        let mut hasher = KnotHasher::new(gen_list());
        hasher.run(&lens);
        let slice = hasher.to_slice();
        println!("Part 1: {}", slice[0] as u64 * slice[1] as u64);
    }

    {
        // Part 2
        println!("Part 2: {}", KnotHasher::hash(&input));
    }
}

fn gen_list() -> Vec<u8> {
    let mut v = Vec::with_capacity(256);
    for i in 0..256 {
        v.push(i as u8);
    }

    v
}

#[test]
fn test_hasher() {
    let mut hasher = KnotHasher::new(vec![0, 1, 2, 3, 4]);
    hasher.run(&vec![3, 4, 1, 5]);
    assert_eq!(hasher.to_slice(), [3, 4, 2, 1, 0]);
}

#[test]
fn test_knot_hash() {
    assert_eq!(KnotHasher::hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(KnotHasher::hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(KnotHasher::hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(KnotHasher::hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}
