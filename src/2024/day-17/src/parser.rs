use itertools::Itertools;
use crate::{Command, Computer, Instr};

pub fn parse(input: &str) -> Computer {
    let mut computer = Computer {
        a: 0,
        b: 0,
        c: 0,
        ip: 0,
        program: vec![],
        raw_program: vec![],
        last_output: None,
    };
    
    // Парсим строки входных данных
    for line in input.lines() {
        if line.starts_with("Register A:") {
            computer.a = line.split(':').nth(1)
                .unwrap_or("0")
                .trim()
                .parse()
                .unwrap_or(0);
        } else if line.starts_with("Register B:") {
            computer.b = line.split(':').nth(1)
                .unwrap_or("0")
                .trim()
                .parse()
                .unwrap_or(0);
        } else if line.starts_with("Register C:") {
            computer.c = line.split(':').nth(1)
                .unwrap_or("0")
                .trim()
                .parse()
                .unwrap_or(0);
        } else if line.starts_with("Program:") {
            // Парсим программу
            computer.program = line.split(':').nth(1)
                .unwrap_or("")
                .trim()
                .split(',')
                .filter_map(|num| num.trim().parse::<i64>().ok())
                .chunks(2)
                .into_iter()
                .map(|chunk| {
                    let nums: Vec<i64> = chunk.collect();
                    let cmd = Command::try_from(nums[0]).unwrap_or(Command::ADV);
                    let op = nums[1];
                    
                    computer.raw_program.extend(nums);
                    
                    match cmd {
                        Command::ADV => Instr::ADV(op),
                        Command::BXL => Instr::BXL(op),
                        Command::BST => Instr::BST(op),
                        Command::JNZ => Instr::JNZ(op),
                        Command::BXC => Instr::BXC(op),
                        Command::OUT => Instr::OUT(op),
                        Command::BDV => Instr::BDV(op),
                        Command::CDV => Instr::CDV(op),
                    }
                })
                .collect();
        }
    }

    computer
}
