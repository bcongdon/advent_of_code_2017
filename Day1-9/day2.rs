use std::cmp;
use std::fs::File;
use std::io::prelude::*;

fn line_difference(line: String) -> i32 {
    let nums = line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    nums.iter().max().unwrap() - nums.iter().min().unwrap()
}

fn checksum(data: String) -> i32 {
    data.split("\n")
        .map(String::from)
        .map(line_difference)
        .sum()
}

fn line_even_score(line: String) -> i32 {
    let nums = line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for (idx, i) in nums.iter().enumerate() {
        for j in nums[idx + 1..].iter() {
            if i % j == 0 || j % i == 0 {
                return cmp::max(i, j) / cmp::min(i, j);
            }
        }
    }
    0
}

fn checksum_2(data: String) -> i32 {
    data.split("\n")
        .map(String::from)
        .map(line_even_score)
        .sum()
}

pub fn main() {
    let filename = String::from("2.txt");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("Part 1: {}", checksum(contents.clone()));
    println!("Part 2: {}", checksum_2(contents));
}
