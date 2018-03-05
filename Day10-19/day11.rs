use std::fs::File;
use std::io::prelude::*;
use std::cmp;

fn manhatten_dist_hex(orig: (i32, i32), dest: (i32, i32)) -> i32 {
    let dist_x = dest.0 - orig.0;
    let dist_y = dest.1 - orig.1;

    if (dist_x > 0) == (dist_y > 0) {
        (dist_x + dist_y).abs() 
    } else {
        cmp::max(dist_x.abs(), dist_y.abs())
    }
}

fn next_loc(orig: (i32, i32), direc: &str) -> (i32, i32) {
    match direc {
        "n" => (orig.0, orig.1 + 1),
        "ne" => (orig.0 + 1, orig.1),
        "se" => (orig.0 + 1, orig.1 - 1),
        "s" => (orig.0, orig.1 - 1),
        "sw" => (orig.0 - 1, orig.1),
        "nw" => (orig.0 - 1, orig.1 + 1),
        _ => panic!("Invalid direction")
    }
}

fn path_dist(directions: Vec<&str>) -> (i32, i32) {
    let mut curr_loc = (0, 0);
    let mut max_dist = 0;

    for direction in directions.iter() {
        curr_loc = next_loc(curr_loc, *direction);
        max_dist = cmp::max(max_dist, manhatten_dist_hex((0, 0), curr_loc))
    }

    (manhatten_dist_hex((0, 0), curr_loc), max_dist)
}


pub fn main() {
    let mut f = File::open("11.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let directions = contents.split(",").collect::<Vec<&str>>();
    let (part1, part2) = path_dist(directions);

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}