use std::fs::File;
use std::io::prelude::*;

struct StreamResult {
    score: usize,
    chars: usize,
}

fn garbage_length(stream: &Vec<char>) -> (usize, usize) {
    let mut chars = 0;
    let mut idx = 1;
    while idx < stream.len() {
        match stream[idx] {
            '>' => break,
            '!' => idx += 2,
            _ => {
                idx += 1;
                chars += 1;
            }
        }
    }
    (idx + 1, chars)
}

fn stream_score(stream: String) -> StreamResult {
    let mut score = 0;
    let mut chars = 0;
    let mut idx = 0;
    let mut stack = vec![1];
    let stream = stream.chars().collect::<Vec<char>>();

    while idx < stream.len() {
        match stream[idx] {
            '{' => {
                let size = stack.len();
                score += stack[size - 1] + 1;
                stack.push(size);
                idx += 1;
            }
            '}' => {
                stack.pop();
                idx += 1;
            }
            '<' => {
                let garbage = stream[idx..].to_vec();
                let (g_len, g_chars) = garbage_length(&garbage);
                idx += g_len;
                chars += g_chars;
            }
            _ => idx += 1,
        }
    }

    StreamResult {
        score: score,
        chars: chars,
    }
}

pub fn main() {
    let filename = String::from("9.txt");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let result = stream_score(contents);
    println!("Part 1: {}", result.score);
    println!("Part 2: {}", result.chars);
}
