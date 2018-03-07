use std::thread;

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const GENERATOR_MOD: u64 = 0x7fffffff;
const CHECK_MASK: u64 = 0x0000ffff;

const PART_1_ITERATIONS: u64 = 40000000;
const PART_2_ITERATIONS: u64 = 5000000;

fn generate(seed: u64, factor: u64) -> u64 {
    let product = seed * factor;
    let tmp = (product & GENERATOR_MOD) + (product >> 31);

    if tmp >> 31 == 0 {
        tmp
    } else {
        tmp - GENERATOR_MOD
    }
}

fn part1(seed_a: u64, seed_b: u64) -> u64 {
    let mut judge = 0;
    let mut a = seed_a;
    let mut b = seed_b;

    for _ in 0..PART_1_ITERATIONS {
        a = generate(a, FACTOR_A);
        b = generate(b, FACTOR_B);

        if a & CHECK_MASK == b & CHECK_MASK {
            judge += 1;
        }
    }

    judge
}

fn part2(seed_a: u64, seed_b: u64) -> u64 {
    let mut judge = 0;
    let mut a = seed_a;
    let mut b = seed_b;

    for _ in 0..PART_2_ITERATIONS {
        a = generate(a, FACTOR_A);
        while a % 4 != 0 {
            a = generate(a, FACTOR_A);
        }
        
        b = generate(b, FACTOR_B);
        while b % 4 != 0 {
            b = generate(b, FACTOR_B);
        }

        if a & CHECK_MASK == b & CHECK_MASK {
            judge += 1;
        }
    }

    judge
}

pub fn main() {
    let seed_a = 783;
    let seed_b = 325;

    let p1_thread = thread::spawn(move || {
        part1(seed_a, seed_b)
    });

    let p2_thread = thread::spawn(move || {
        part2(seed_a, seed_b)
    });

    let p1_result = p1_thread.join().unwrap();
    let p2_result = p2_thread.join().unwrap();

    println!("Part 1: {}", p1_result);
    println!("Part 2: {}", p2_result);
}