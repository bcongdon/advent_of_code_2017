use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp;

struct Instruction {
    assignment_reg: String,
    assignment_op: String,
    assignment_amt: i32,
    compare_reg: String,
    compare_operator: String,
    compare_amt: i32,
}

impl Instruction {
    fn from_string(input: String) -> Instruction {
        let fields = input
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        Instruction {
            assignment_reg: fields[0].clone(),
            assignment_op: fields[1].clone(),
            assignment_amt: fields[2].parse::<i32>().unwrap(),
            compare_reg: fields[4].clone(),
            compare_operator: fields[5].clone(),
            compare_amt: fields[6].parse::<i32>().unwrap(),
        }
    }
}

fn assignment_op(operator: &String, a: i32, b: i32) -> i32 {
    match operator.as_ref() {
        "inc" => a + b,
        "dec" => a - b,
        _ => panic!("Invalid operator: {}", operator),
    }
}

fn compare_op(operator: &String, a: i32, b: i32) -> bool {
    match operator.as_ref() {
        ">" => a > b,
        "<" => a < b,
        ">=" => a >= b,
        "<=" => a <= b,
        "==" => a == b,
        "!=" => a != b,
        _ => panic!("Invalid comparison operator: {}", operator),
    }
}

fn run_interpreter(instructions: Vec<Instruction>) -> (i32, i32) {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max_at_runtime = 0;
    let mut max_at_end = 0;

    for inst in instructions.iter() {
        let cmp_reg_val = *registers.get(&inst.compare_reg).unwrap_or(&0);

        if compare_op(&inst.compare_operator, cmp_reg_val, inst.compare_amt) {
            let asn_reg_val = *registers.get(&inst.assignment_reg).unwrap_or(&0);
            let asn_new_val = assignment_op(&inst.assignment_op, asn_reg_val, inst.assignment_amt);
            registers.insert(inst.assignment_reg.clone(), asn_new_val);

            max_at_runtime = cmp::max(max_at_runtime, asn_new_val);
        }
    }

    for (_, val) in registers.iter() {
        max_at_end = cmp::max(max_at_end, *val);
    }

    (max_at_runtime, max_at_end)
}

pub fn main() {
    let filename = String::from("8.txt");
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let instructions = contents
        .split("\n")
        .map(String::from)
        .map(Instruction::from_string)
        .collect::<Vec<Instruction>>();

    let (max_runtime, max_end) = run_interpreter(instructions);
    println!("Part 1: {}", max_end);
    println!("Part 2: {}", max_runtime);
}
