use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct Program {
    pid: String,
    weight: usize,
    children: Vec<String>,
}

impl Program {
    fn from_string(input: String) -> Program {
        let fields = input
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        let pid = fields[1]
            .chars()
            .filter(|x| match *x {
                ')' => false,
                '(' => false,
                _ => true,
            })
            .collect::<String>();

        let children: Vec<String>;
        if fields.len() > 2 {
            children = fields[3..]
                .iter()
                .map(|x| x.replace(",", ""))
                .collect::<Vec<String>>();
        } else {
            children = Vec::new();
        }

        Program {
            pid: fields[0].clone(),
            weight: pid.parse::<usize>().unwrap(),
            children,
        }
    }
}

fn find_source(programs: Vec<Program>) -> Program {
    let has_parent = programs
        .iter()
        .flat_map(|x| x.children.clone())
        .collect::<HashSet<String>>();
    programs
        .iter()
        .filter(|x| !has_parent.contains(&x.pid))
        .next()
        .unwrap()
        .clone()
}

fn balance(
    root: &Program,
    vertices: &HashMap<String, Program>,
    weights: &mut HashMap<String, usize>,
) -> usize {
    for child in root.children.clone() {
        let fixed = balance(vertices.get(&child).unwrap(), vertices, weights);
        if fixed != <usize>::max_value() {
            return fixed;
        }
    }

    let mut cost = <usize>::max_value();
    for child in root.children.clone() {
        if weights[&child] < cost {
            cost = weights[&child]
        }
    }
    for child in root.children.clone() {
        if weights[&child] != cost {
            let child_cost = vertices[&child]
                .children
                .iter()
                .map(|x| weights[x])
                .sum::<usize>();
            let fixed = cost - child_cost;
            return fixed;
        }
    }
    weights.insert(
        root.pid.clone(),
        root.weight + root.children.len() * cost as usize,
    );

    <usize>::max_value()
}

fn balance_tree(programs: Vec<Program>, root: Program) -> usize {
    let mut vertices = HashMap::new();
    let mut weights = HashMap::new();

    for program in programs {
        vertices.insert(program.pid.clone(), program.clone());
        weights.insert(program.pid.clone(), program.weight);
    }
    balance(&root, &vertices, &mut weights)
}

pub fn main() {
    let filename = String::from("7.txt");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let programs = contents
        .split("\n")
        .map(String::from)
        .map(Program::from_string)
        .collect::<Vec<Program>>();

    let source = find_source(programs.clone());
    println!("Part 1: {}", source.pid);

    let fixed = balance_tree(programs, source);
    println!("Part 2: {}", fixed);
}
