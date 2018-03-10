use std::collections::VecDeque;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

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

    send_flag: bool,
    send_val: i64,

    receive_queue: VecDeque<i64>,

    part1: bool,
    first_received_flag: bool,
    first_received: i64,
    last_played: i64,
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
            "snd" => {
                self.last_played = self.get_value(&inst.operand0);
                self.send_val = self.last_played;
                self.send_flag = true;
            }
            "set" => {
                let new_val = self.get_value(&inst.operand1);
                self.registers.insert(op0_key, new_val);
            }
            "add" => {
                let prev_val = self.get_value(&inst.operand0);
                let new_val = prev_val + self.get_value(&inst.operand1);
                self.registers.insert(op0_key, new_val);
            }
            "mul" => {
                let prev_val = self.get_value(&inst.operand0);
                let new_val = prev_val * self.get_value(&inst.operand1);
                self.registers.insert(op0_key, new_val);
            }
            "mod" => {
                let mut prev_val = self.get_value(&inst.operand0);
                if self.get_value(&inst.operand1) != 0 {
                    prev_val = prev_val.abs();
                    let new_val = prev_val % self.get_value(&inst.operand1);
                    self.registers.insert(op0_key, new_val);
                }
            }
            "rcv" => {
                if self.part1 {
                    if self.get_value(&inst.operand0) != 0 {
                        self.first_received = self.last_played;
                        self.first_received_flag = true;
                    }
                } else if self.receive_queue.len() > 0 {
                    let recv_val = self.receive_queue.pop_front().unwrap();
                    self.registers.insert(op0_key, recv_val);
                } else {
                    return;
                }
            }
            "jgz" => {
                if self.get_value(&inst.operand0) > 0 {
                    self.pc = (self.pc as i64 + self.get_value(&inst.operand1)) as usize;
                    return;
                }
            }
            _ => panic!("Unhandled instruction: {}", inst.operation),
        }
        self.pc += 1
    }

    fn make_process_register(pid: usize) -> HashMap<char, i64> {
        let mut register = HashMap::new();
        register.insert('p', pid as i64);

        for i in 0..255 {
            let key = char::from(i);
            register.insert(key, 0);
        }

        register
    }

    fn new(instructions: Vec<Instruction>, pid: usize, part1: bool) -> VM {
        VM {
            instructions: instructions,
            pc: 0,
            registers: VM::make_process_register(pid),

            send_flag: false,
            send_val: 0,

            receive_queue: VecDeque::new(),

            part1: part1,
            first_received_flag: false,
            first_received: 0,
            last_played: 0,
        }
    }
}

fn run_vm_pair(instructions: Vec<Instruction>) -> i32 {
    let mut p0 = VM::new(instructions.clone(), 0, false);
    let mut p1 = VM::new(instructions.clone(), 1, false);

    let mut part2 = 0;
    let mut pc0 = usize::max_value();
    let mut pc1 = usize::max_value();

    while p0.pc != pc0 || p1.pc != pc1 {
        pc0 = p0.pc;
        pc1 = p1.pc;

        if p0.send_flag {
            p1.receive_queue.push_back(p0.send_val);
            p0.send_flag = false;
        }
        if p1.send_flag {
            p0.receive_queue.push_back(p1.send_val);
            p1.send_flag = false;
            part2 += 1;
        }

        p0.run_instruction();
        p1.run_instruction();
    }

    part2
}

pub fn main() {
    let mut f = File::open("18.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let instructions = contents
        .split("\n")
        .map(|x| String::from(x))
        .map(|x| Instruction::from_string(x))
        .collect::<Vec<Instruction>>();

    let mut p0 = VM::new(instructions.clone(), 0, true);
    while !p0.first_received_flag || p0.first_received == 0 {
        p0.run_instruction();
    }
    println!("Part 1: {}", p0.first_received);

    let part2 = run_vm_pair(instructions.clone());
    println!("Part 2: {}", part2);
}
