use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn cycles_until_reseen(bins: Vec<i32>) -> (usize, Vec<i32>) {
    let mut configs: HashSet<Vec<i32>> = HashSet::new();
    let mut curr = bins.clone();

    while !configs.contains(&curr) {
        configs.insert(curr.clone());

        let min_idx = curr.iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, item)| item)
            .unwrap()
            .0;

        let redistribute = curr[min_idx];
        curr[min_idx] = 0;

        let all_gains = redistribute / bins.len() as i32;
        let partial_gain_bins = redistribute % bins.len() as i32;
        for i in 0..bins.len() {
            curr[i] += all_gains;
            if i < partial_gain_bins as usize {
                curr[(min_idx + i + 1) % bins.len()] += 1
            }
        }
    }

    (configs.len(), curr)
}

pub fn main() {
    let filename = String::from("6.txt");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let bins = contents
        .split_whitespace()
        .map(String::from)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (num_cycles, last_seen) = cycles_until_reseen(bins);
    let (loop_cycles, _) = cycles_until_reseen(last_seen);
    println!("Part 1: {}", num_cycles);
    println!("Part 2: {}", loop_cycles);
}
