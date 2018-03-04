use std::fs::File;
use std::io::prelude::*;

fn main() {
    let puzzle = get_puzzle_input("1.txt");

    assert!(captcha1("1122") == 3);
    println!("Part 1: {}", captcha1(&puzzle));
    println!("Part 2: {}", captcha2(&puzzle));
}

fn get_puzzle_input(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.replace("\n", "")
}

fn captcha1(input: &str) -> u32 {
    let mut counter = 0;
    let bytes = input.as_bytes();
    for (i, c) in bytes.iter().enumerate() {
        let next_idx = (i + 1) % input.len();
        if bytes[next_idx] == *c {
            counter += (*c as char).to_digit(10).unwrap();
        }
    }
    counter
}

fn captcha2(input: &str) -> u32 {
    let mut counter = 0;
    let bytes = input.as_bytes();
    for (i, c) in bytes.iter().enumerate() {
        let next_idx = (i + input.len() / 2) % input.len();
        if bytes[next_idx] == *c {
            counter += (*c as char).to_digit(10).unwrap();
        }
    }
    counter
}
