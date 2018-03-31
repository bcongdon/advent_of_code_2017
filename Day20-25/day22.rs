use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug)]
enum State {
    Clean,
    Infected,
    Flagged,
    Weakened,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Location {
    x: i32,
    y: i32,
}

type Direction = i32;

fn turn_left(d: Direction) -> Direction {
    (d + 3) % 4
}

fn turn_right(d: Direction) -> Direction {
    (d + 1) % 4
}

fn turn_around(d: Direction) -> Direction {
    (d + 2) % 4
}

const UP: Direction = 0;
const RIGHT: Direction = 1;
const DOWN: Direction = 2;
const LEFT: Direction = 3;

type Grid = HashMap<Location, State>;
type InfectionTransition = HashMap<State, State>;
type DirectionTransition = Fn(State, Direction) -> Direction;

fn run_infection(
    mut grid: Grid,
    start: Location,
    iterations: usize,
    inf: InfectionTransition,
    dir: &DirectionTransition,
) -> usize {
    let mut coord = start;
    let mut direction = UP;
    let mut newly_infected: usize = 0;

    for _ in 0..iterations {
        let state = grid.get(&coord).cloned().unwrap_or(State::Clean);
        direction = dir(state, direction);
        let new_state = inf.get(&state).unwrap();

        grid.insert(coord, *new_state);

        if *new_state == State::Infected {
            newly_infected += 1;
        }

        match direction {
            UP => coord.y -= 1,
            DOWN => coord.y += 1,
            RIGHT => coord.x += 1,
            LEFT => coord.x -= 1,
            _ => panic!("Invalid direction"),
        };
    }

    newly_infected
}

fn create_part1_infection_transition() -> InfectionTransition {
    let mut map = HashMap::new();
    map.insert(State::Clean, State::Infected);
    map.insert(State::Infected, State::Clean);
    map
}

fn create_part2_infection_transition() -> InfectionTransition {
    let mut map = HashMap::new();
    map.insert(State::Clean, State::Weakened);
    map.insert(State::Weakened, State::Infected);
    map.insert(State::Infected, State::Flagged);
    map.insert(State::Flagged, State::Clean);
    map
}

fn part1_direction_transition(s: State, d: Direction) -> Direction {
    match s {
        State::Infected => turn_right(d),
        State::Clean => turn_left(d),
        _ => panic!("invalid state"),
    }
}

fn part2_direction_transition(s: State, d: Direction) -> Direction {
    match s {
        State::Clean => turn_left(d),
        State::Weakened => d,
        State::Infected => turn_right(d),
        State::Flagged => turn_around(d),
    }
}

pub fn main() {
    let mut f = File::open("22.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>();
    let mut grid: HashMap<Location, State> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let loc = Location {
                x: j as i32,
                y: i as i32,
            };
            if c == '#' {
                grid.insert(loc, State::Infected);
            }
        }
    }

    let start = Location {
        x: (lines.len() / 2) as i32,
        y: (lines.len() / 2) as i32,
    };

    let mut part1_grid = grid.clone();
    part1_grid.reserve(10000);
    let part1 = run_infection(
        part1_grid,
        start,
        10000,
        create_part1_infection_transition(),
        &part1_direction_transition,
    );
    println!("Part 1: {}", part1);

    let mut part2_grid = grid.clone();
    part2_grid.reserve(10000000);
    let part2 = run_infection(
        part2_grid,
        start,
        10000000,
        create_part2_infection_transition(),
        &part2_direction_transition,
    );
    println!("Part 2: {}", part2);
}
