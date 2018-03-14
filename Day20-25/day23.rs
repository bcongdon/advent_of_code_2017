use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::f64;

#[derive(Debug, Clone)]
struct Instruction {
    operation: String,
    operand0: String,
    operand1: String,
}

impl Instruction {
    fn from_string(value: String) -> Instruction {
        let fields = value.split(" ").collect::<Vec<&str>>();
        if (fields.len() != 2) && (fields.len() != 3) {
            panic!("Invalid instruction: {}", value);
        }

        let mut inst = Instruction {
            operation: String::from(fields[0]),
            operand0: String::from(fields[1]),
            operand1: String::from(""),
        };

        if fields.len() == 3 {
            inst.operand1 = String::from(fields[2])
        }

        inst
    }
}

#[derive(Debug)]
struct VM {
    instructions: Vec<Instruction>,
    pc: usize,
    registers: HashMap<char, i64>,

    mult_operations: u32,
}

impl VM {
    fn get_value(&self, token: &String) -> i64 {
        match token.parse::<i64>() {
            Err(_) => *self.registers.get(&token.chars().next().unwrap()).unwrap(),
            Ok(val) => val,
        }
    }

    fn run_instruction(&mut self) {
        let inst = &self.instructions[self.pc];
        let op0_key = inst.operand0.chars().next().unwrap();

        match inst.operation.as_ref() {
            "set" => {
                let new_val = self.get_value(&inst.operand1);
                self.registers.insert(op0_key, new_val);
            }
            "sub" => {
                let prev_val = self.get_value(&inst.operand0);
                let new_val = prev_val - self.get_value(&inst.operand1);
                self.registers.insert(op0_key, new_val);
            }
            "mul" => {
                let prev_val = self.get_value(&inst.operand0);
                let new_val = prev_val * self.get_value(&inst.operand1);
                self.registers.insert(op0_key, new_val);
                self.mult_operations += 1;
            }
            "jnz" => {
                if self.get_value(&inst.operand0) != 0 {
                    self.pc = (self.pc as i64 + self.get_value(&inst.operand1)) as usize;
                    return;
                }
            }
            _ => panic!("Unhandled instruction: {}", inst.operation),
        }
        self.pc += 1
    }

    fn make_process_register() -> HashMap<char, i64> {
        let mut register = HashMap::new();

        for i in 0..255 {
            let key = char::from(i);
            register.insert(key, 0);
        }

        register
    }

    fn new(instructions: Vec<Instruction>) -> VM {
        VM {
            instructions: instructions,
            pc: 0,
            registers: VM::make_process_register(),

            mult_operations: 0,
        }
    }
}

fn part2(instructions: Vec<Instruction>) -> i32 {
    let (mut start, mut end, mut skip) = (0, 0, 0);
    for inst in instructions.iter() {
        if inst.operation == "set" && inst.operand0 == "b" {
            let val = inst.operand1.parse::<i32>().unwrap();
            start = val * 100 + 100000;
        } else if inst.operation == "sub" && inst.operand0 == "c" {
            let val = inst.operand1.parse::<i32>().unwrap();
            end = start - val;
        } else if inst.operation == "sub" && inst.operand0 == "b" && end != 0 {
            let val = inst.operand1.parse::<i32>().unwrap();
            skip = -val;
        }
    }

    let mut h = 0;
    let mut idx = start;
    while idx <= end {
        let sqrt_x = (idx as f64).sqrt() as i32;
        for j in 2..sqrt_x {
            if idx % j == 0 {
                h += 1;
            }
        }

        idx += skip;
    }

    h
}

pub fn main() {
    let mut f = File::open("23.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let instructions = contents
        .split("\n")
        .map(|x| String::from(x))
        .map(|x| Instruction::from_string(x))
        .collect::<Vec<Instruction>>();

    let mut p0 = VM::new(instructions.clone());
    while p0.pc < instructions.len() {
        p0.run_instruction();
    }
    println!("Part 1: {}", p0.mult_operations);

    println!("Part 2: {}", part2(instructions));
}
