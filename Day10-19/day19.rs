use std::fs::File;
use std::io::prelude::*;

fn is_inflection_point(c: char) -> bool {
    c == '|' || c == '-' || c.is_ascii_alphanumeric()
}

fn traverse_pipe_grid(grid: Vec<Vec<char>>) -> (String, i32) {
    let start = grid[0].iter()
        .enumerate()
        .filter(|&(_, x)| *x == '|')
        .map(|(i, _)| i)
        .next().unwrap();

    let (mut dx, mut dy): (i32, i32) = (0, 1);
    let (mut x, mut y) = (start, 0);
    let mut letter_buf: Vec<char> = Vec::new();
    let mut count = 0;

    while !grid[y][x].is_whitespace() {
        let curr_char = grid[y][x];
        if curr_char == '+' {
            if dx != 0 {
                dx = 0;
                if y+1 < grid.len() && is_inflection_point(grid[y+1][x]) {
                    dy = 1;
                } else {
                    dy = -1;
                }
            } else {
                dy = 0;
                if x+1 < grid[0].len() && is_inflection_point(grid[y][x+1]) {
                    dx = 1;
                } else {
                    dx = -1;
                }
            }
        } else if curr_char.is_ascii_alphanumeric() {
            letter_buf.push(curr_char);
        }
        x = ((x as i32) + dx) as usize;
        y = ((y as i32) + dy) as usize;
        count += 1;
    }

    (letter_buf.iter().cloned().collect(), count)
}

fn make_grid(lines: Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

pub fn main() {
    let mut f = File::open("19.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents.split("\n").map(|x| String::from(x)).collect::<Vec<String>>();
    let grid = make_grid(lines);
    let (letters, count) = traverse_pipe_grid(grid);

    println!("Part 1: {}", letters);
    println!("Part 2: {}", count);
}