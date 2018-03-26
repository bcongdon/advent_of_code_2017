use std::iter;
use std::fs::File;
use std::io::prelude::*;

fn parse_line(line: String) -> (usize, usize) {
    let fields = line.split(": ").map(String::from).collect::<Vec<String>>();
    let depth = fields[0].parse::<usize>().unwrap();
    let range = fields[1].parse::<usize>().unwrap();
    (depth, range)
}

fn parse_ranges(lines: Vec<String>) -> Vec<usize> {
    let (max_depth, _) = parse_line(lines[lines.len() - 1].clone());
    let mut out = iter::repeat(0).take(max_depth + 1).collect::<Vec<usize>>();

    for line in lines {
        let (depth, range) = parse_line(line);
        out[depth] = range;
    }

    out
}

fn trip_severity(ranges: &Vec<usize>) -> usize {
    ranges
        .iter()
        .enumerate()
        .map(|(time, &range)| {
            if range != 0 && time % (2 * range - 2) == 0 {
                return time * range;
            }
            0
        })
        .sum::<usize>()
}

fn is_caught(ranges: &Vec<usize>, delay: usize) -> bool {
    ranges
        .iter()
        .enumerate()
        .any(|(time, &range)| range != 0 && (time + delay) % (2 * range - 2) == 0)
}

fn min_delay(ranges: Vec<usize>) -> usize {
    (0..)
        .filter(|&delay| !is_caught(&ranges, delay))
        .next()
        .unwrap()
}

pub fn main() {
    let mut f = File::open("13.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let ranges = parse_ranges(contents.split("\n").map(String::from).collect());
    let part1 = trip_severity(&ranges);
    println!("Part 1: {}", part1);

    let part2 = min_delay(ranges);
    println!("Part 2: {}", part2);
}
