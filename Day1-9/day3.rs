use std::cmp;
use std::collections::HashMap;

fn next_location(loc: (i32, i32)) -> (i32, i32) {
    let det = cmp::max(loc.0.abs(), loc.1.abs());

    let (x, y) = loc;
    if det == x && det == -y {
        return (x + 1, y);
    } else if det == x && det != y {
        return (x, y + 1);
    } else if det == y && det != -x {
        return (x - 1, y);
    } else if det == -x && det != -y {
        return (x, y - 1);
    } else if det == -y {
        return (x + 1, y);
    }
    panic!("Unhandled case!");
}

fn spiral_cost(n: u32) -> i32 {
    let mut loc = (1, 0);

    for _ in 2..n {
        loc = next_location(loc)
    }

    loc.0.abs() + loc.1.abs()
}

fn spiral_2(n: u32) -> i32 {
    let mut mem = HashMap::new();
    mem.insert((0, 0), 1);
    let mut loc = (1, 0);

    let mut cost = 0;
    while cost <= n {
        cost = 0;
        for i in -1..2 {
            for j in -1..2 {
                cost += mem.get(&(loc.0 + i, loc.1 + j)).unwrap_or(&0);
            }
        }
        mem.insert(loc, cost);
        loc = next_location(loc);
    }
    cost as i32
}

pub fn main() {
    println!("Part 1: {}", spiral_cost(312051));
    println!("Part 2: {}", spiral_2(312051))
}
