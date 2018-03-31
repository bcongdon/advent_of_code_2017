use std::fs::File;
use std::io::prelude::*;
use std::cmp;

// Solution inspired by https://www.reddit.com/r/adventofcode/comments/7lte5z/2017_day_24_solutions/drq01ne/

#[derive(Clone, PartialEq)]
struct Component {
    pins: (usize, usize),
}

impl Component {
    fn from(input: String) -> Component {
        let nums = input.split("/").map(String::from).collect::<Vec<String>>();
        let a = nums[0].parse::<usize>().unwrap();
        let b = nums[1].parse::<usize>().unwrap();

        Component { pins: (a, b) }
    }
}

fn component_score(components: &Vec<Component>) -> usize {
    components
        .iter()
        .fold(0, |acc, comp| acc + comp.pins.0 + comp.pins.1)
}

fn bridge_finder<F>(components: &Vec<Component>, next: usize, cmp_fn: &F) -> Vec<Component>
where
    F: Fn(&Vec<Component>, &Vec<Component>) -> cmp::Ordering,
{
    components
        .iter()
        .enumerate()
        .filter(|&(_, ref comp)| comp.pins.0 == next || comp.pins.1 == next)
        .map(|(idx, comp)| {
            let mut components_clone = components.clone();
            components_clone.swap_remove(idx);
            let other = if comp.pins.0 == next {
                comp.pins.1
            } else {
                comp.pins.0
            };
            let mut v = bridge_finder(&components_clone, other, cmp_fn);
            v.push(comp.clone());
            v
        })
        .max_by(cmp_fn)
        .unwrap_or(Vec::new())
}

fn find_longest(components: &Vec<Component>, next: usize) -> Vec<Component> {
    bridge_finder(
        components,
        next,
        &|a: &Vec<Component>, b: &Vec<Component>| {
            a.len()
                .cmp(&b.len())
                .then(component_score(a).cmp(&component_score(b)))
        },
    )
}

fn find_strongest(components: &Vec<Component>, next: usize) -> Vec<Component> {
    bridge_finder(
        components,
        next,
        &|a: &Vec<Component>, b: &Vec<Component>| component_score(a).cmp(&component_score(b)),
    )
}

pub fn main() {
    let mut f = File::open("24.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let components = contents
        .split("\n")
        .map(String::from)
        .map(Component::from)
        .collect::<Vec<Component>>();

    let part1 = find_strongest(&components, 0);
    println!("Part 1: {}", component_score(&part1));

    let part2 = find_longest(&components, 0);
    println!("Part 2: {}", component_score(&part2));
}
