use std::fs::File;
use std::io::prelude::*;

fn first_jump_outside(jumps: Vec<i32>, mutator: fn(i32) -> i32) -> usize {
    let mut c = 0;
    let mut steps = 0;
    let mut jumps = jumps.clone();

    while c < jumps.len() {
        let curr_jump = jumps[c];
        jumps[c] = mutator(curr_jump);
        steps += 1;
        c = (c as i32 + curr_jump) as usize;
    }

    steps
}

pub fn main() {
    let filename = String::from("5.txt");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let jumps = contents
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let part1 = first_jump_outside(jumps.clone(), |x| x + 1);
    println!("Part 1: {}", part1);

    let part2 = first_jump_outside(jumps, |x| if x >= 3 { x - 1 } else { x + 1 });
    println!("Part 2: {}", part2);
}
