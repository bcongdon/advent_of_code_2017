const NUM_DANCERS: u32 = 16;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs::File;
use std::io::prelude::*;
use std::thread;


fn run_dance(steps: &Vec<String>, dancer_input: Vec<char>, repetitions: u32) -> String {
    let mut seen_states: HashSet<String> = HashSet::new();
    let mut ordered_states: Vec<String> = Vec::new();
    let mut dancers = dancer_input.clone();

    for i in 0..repetitions {
        let dancer_str = String::from_iter(dancers.clone());
        if seen_states.contains(&dancer_str) {
            return ordered_states[(repetitions % i) as usize].clone();
        } else {
            seen_states.insert(dancer_str);
            ordered_states.push(String::from_iter(dancers.clone()));
        }

        for step in steps.iter() {
            match (*step).chars().next().unwrap() {
                's' => {
                    let shift = step[1..].parse::<usize>().unwrap();
                    let clone = dancers.clone();
                    for i in 0..clone.len() {
                        dancers[(i + shift) % clone.len()] = clone[i];
                    }
                }
                'x' => {
                    let positions = step[1..].split("/").collect::<Vec<_>>();
                    let p1 = positions[0].parse::<usize>().unwrap();
                    let p2 = positions[1].parse::<usize>().unwrap();
                    
                    let tmp = dancers[p1];
                    dancers[p1] = dancers[p2];
                    dancers[p2] = tmp;
                }
                'p' => {
                    let partners = step[1..].split("/").collect::<Vec<_>>();
                    let (mut p1, mut p2) = (1000, 1000);

                    for j in 0..dancers.len() {
                        if partners[0].chars().next().unwrap() == dancers[j] {
                            p1 = j;
                        }
                        if partners[1].chars().next().unwrap() == dancers[j] {
                            p2 = j;
                        }
                    }
                    let tmp = dancers[p1];
                    dancers[p1] = dancers[p2];
                    dancers[p2] = tmp;
                }
                _ => {panic!("Invalid step: {}", step)}
            }
        }
    }

    dancers.iter().cloned().collect()
}

fn make_dancers(n: u32) -> Vec<char> {
    let mut dancers = Vec::new();
    for i in 0..n {
        dancers.push((97 + i as u8) as char);
    }
    dancers
}

fn get_steps() -> Vec<String> {
    let mut f = File::open("16.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.split(",").map(|x| String::from(x)).collect::<Vec<String>>()
}

pub fn main() {    
    let p1_thread = thread::spawn(move || {
        let dancers = make_dancers(NUM_DANCERS);
        run_dance(&get_steps(), dancers, 1)
    });

    let p2_thread = thread::spawn(move || {
        let dancers = make_dancers(NUM_DANCERS);
        run_dance(&get_steps(), dancers, 1000000000)
    });

    let p1_result = p1_thread.join().unwrap();
    let p2_result = p2_thread.join().unwrap();
    
    println!("Part 1: {}", p1_result);
    println!("Part 2: {}", p2_result);
}