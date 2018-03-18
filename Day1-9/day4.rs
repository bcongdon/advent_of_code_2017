use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn validate_part1(pw: String) -> bool {
    let words = pw.split_whitespace().map(String::from).collect::<Vec<_>>();
    let uniq = words.iter().cloned().collect::<HashSet<String>>();
    words.len() == uniq.len()
}

fn validate_part2(pw: String) -> bool {
    let words = pw.split_whitespace().map(String::from).collect::<Vec<_>>();
    let uniq_sorted = words
        .iter()
        .cloned()
        .map(|x| {
            let mut split_chars = x.clone().chars().collect::<Vec<_>>();
            split_chars.sort();
            split_chars.iter().collect::<String>()
        })
        .collect::<HashSet<String>>();

    words.len() == uniq_sorted.len()
}

fn count_valid_passwords(lines: Vec<String>, validator: fn(String) -> bool) -> i32 {
    lines.iter().fold(0, |acc, line| {
        acc + (if validator(line.clone()) { 1 } else { 0 })
    })
}

pub fn main() {
    let filename = String::from("4.txt");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents.split("\n").map(String::from).collect::<Vec<_>>();

    let part1 = count_valid_passwords(lines.clone(), validate_part1);
    println!("Part 1: {}", part1);

    let part2 = count_valid_passwords(lines, validate_part2);
    println!("Part 2: {}", part2);
}
