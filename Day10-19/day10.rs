use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    let mut f = File::open("10.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let puzzle_lengths = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut hash = KnotHash::new(256);

    hash.run_iter(puzzle_lengths.as_slice());
    let part1_result = hash.hash.get(0).unwrap() * hash.hash.get(1).unwrap();
    println!("Part 1: {}", part1_result);

    let byte_contents = contents.as_bytes().iter().map(|&x| x as i32).collect::<Vec<i32>>();
    println!("{:?}", byte_contents);
    let part2_result = knot_hash_ascii(byte_contents.as_slice(), 64);
    println!("Part 2: {}", part2_result);
}

static HASH_SUFFIX: &'static [i32] = &[17, 31, 73, 47, 23];

struct KnotHash {
    hash: Vec<i32>,
    curr: i32,
    skip: i32,
}

impl KnotHash {
    fn new(size: usize) -> KnotHash {
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            vec.insert(i, i as i32);
        }

        KnotHash {
            hash: vec,
            curr: 0,
            skip: 0
        }
    }

    fn run_iter(&mut self, lengths: &[i32]) {
        let size = self.hash.len();

        for curr_len in lengths.iter() {
            for i in 0..(curr_len / 2) {
                let s = (self.curr + i) % (size as i32);
                let e = (self.curr + curr_len - i - 1) % (size as i32);

                let tmp = self.hash[s as usize];
                self.hash[s as usize] = self.hash[e as usize];
                self.hash[e as usize] = tmp;
            }

            self.curr = (self.curr + curr_len + self.skip) % (size as i32);
            self.skip += 1;
        }
    }
}

fn knot_hash_ascii(lengths: &[i32], rounds: i32) -> String {
    let mut knot_hash = KnotHash::new(256);
    let mut prepared_bytes = lengths.to_vec();
    prepared_bytes.extend_from_slice(HASH_SUFFIX);

    let lengths_slice = prepared_bytes.as_slice();
    for _ in 0..rounds {
        knot_hash.run_iter(lengths_slice);
    }

    let dense_hash = &mut Vec::with_capacity(16);
    for i in 0..16 {
        let mut xor: u8 = 0;

        for j in 16*i .. 16*i + 16 {
            xor ^= (*knot_hash.hash.get(j).unwrap()) as u8;
        }
        dense_hash.insert(i, xor);
    }    
    dense_hash.iter().map(|x| format!("{:x}", x)).collect::<Vec<String>>().concat()
}