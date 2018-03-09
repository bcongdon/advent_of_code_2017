const PUZZLE_INPUT: i32 = 354;

fn run_spinlock(offset: i32, iterations: i32) -> i32 {
    let mut spinlock: Vec<i32> = Vec::new();
    spinlock.push(0);

    let mut loc = 0;
    for i in 1 .. iterations+1 {
        loc = (loc + offset)%i + 1;
        spinlock.insert(loc as usize, i);
    }

    spinlock[(loc+1) as usize]
}

fn simulate_spinlock(offset: i32, iterations: i32) -> i32 {
    (1..iterations+1).scan(0, |loc, i| {
        *loc = (*loc + offset)%i + 1;
        Some(*loc)
    }).enumerate()
      .filter(|&(_, loc)| loc == 1)
      .map(|(i, _)| (i + 1) as i32)
      .last()
      .unwrap()
}

pub fn main() {
    let part1 = run_spinlock(PUZZLE_INPUT, 2017);
    println!("Part 1: {}", part1);

    let part2 = simulate_spinlock(PUZZLE_INPUT, 50000000);
    println!("Part 2: {}", part2);
}