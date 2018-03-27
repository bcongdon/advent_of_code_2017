use std::collections::HashSet;

static HASH_SUFFIX: &'static [i32] = &[17, 31, 73, 47, 23];
const GRID_SIZE: usize = 128;

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
            skip: 0,
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

fn knot_hash_ascii(lengths: &[i32], rounds: i32) -> Vec<u8> {
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

        for j in 16 * i..16 * i + 16 {
            xor ^= (*knot_hash.hash.get(j).unwrap()) as u8;
        }
        dense_hash.insert(i, xor);
    }
    (*dense_hash).to_vec()
}

fn construct_grid(seed: String) -> [[u8; GRID_SIZE]; GRID_SIZE] {
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];

    for i in 0..GRID_SIZE {
        let key = format!("{}-{}", seed, i)
            .chars()
            .map(|x| (x as u8) as i32)
            .collect::<Vec<i32>>();
        let row = knot_hash_ascii(key.as_slice(), 64);
        let mut row_slice: [u8; GRID_SIZE] = [0; GRID_SIZE];

        for (idx, val) in row.iter().enumerate() {
            row_slice[idx] = *val as u8;
        }

        grid[i] = row_slice;
    }

    grid
}

fn is_set(grid: &[[u8; GRID_SIZE]; GRID_SIZE], l: &Location) -> bool {
    let row = grid[l.x as usize];
    let bit = (row[l.y as usize / 8]) >> (7 - l.y % 8) & 0x01;
    bit == 0x1
}

fn dfs(grid: &[[u8; GRID_SIZE]; GRID_SIZE], l: Location, visited: &mut HashSet<Location>) {
    if visited.contains(&l.clone()) || !is_set(grid, &l) {
        return;
    }

    visited.insert(l.clone());
    if l.x < GRID_SIZE - 1 {
        dfs(grid, Location { x: l.x + 1, y: l.y }, visited)
    }
    if l.y < GRID_SIZE - 1 {
        dfs(grid, Location { x: l.x, y: l.y + 1 }, visited)
    }
    if l.x > 0 {
        dfs(grid, Location { x: l.x - 1, y: l.y }, visited)
    }
    if l.y > 0 {
        dfs(grid, Location { x: l.x, y: l.y - 1 }, visited)
    }
}

fn count_groups(grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> usize {
    let mut visited: HashSet<Location> = HashSet::new();
    let mut group_count = 0;

    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            let l = Location { x, y };
            if visited.contains(&l) || !is_set(grid, &l) {
                continue;
            }
            dfs(grid, l, &mut visited);
            group_count += 1;
        }
    }

    group_count
}

fn bit_set_count(v: u8) -> usize {
    let mut v = (v & 0x55) + ((v >> 1) & 0x55);
    v = (v & 0x33) + ((v >> 2) & 0x33);
    ((v + (v >> 4)) & 0xF) as usize
}

fn count_used(grid: [[u8; GRID_SIZE]; GRID_SIZE]) -> usize {
    let mut count = 0;

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            count += bit_set_count(grid[i][j])
        }
    }

    count
}

pub fn main() {
    let puzzle_input = String::from("uugsqrei");
    let grid = construct_grid(puzzle_input);

    let part1 = count_used(grid.clone());
    println!("Part 1: {}", part1);

    let part2 = count_groups(&grid);
    println!("Part 2: {}", part2);
}
