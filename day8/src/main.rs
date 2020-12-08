use std::error::Error;
use std::io::{self, Read};
use std::collections::HashSet;

type Word = i64;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Instruction {
    Acc(Word),
    Jmp(Word),
    Nop(Word)
}

fn parse_instruction(text: &str) -> Result<Instruction, Box<dyn Error>> {
    let parts = text.split_whitespace().collect::<Vec<_>>();
    let instr = parts.get(0).ok_or("Invalid instruction.")?;
    match *instr {
        "acc" => {
            let arg = parts.get(1).ok_or("Invalid acc instruction.")?;
            Ok(Instruction::Acc(arg.trim_start_matches('+').parse()?))
        }
        "jmp" => {
            let arg = parts.get(1).ok_or("Invalid jmp instruction.")?;
            Ok(Instruction::Jmp(arg.trim_start_matches('+').parse()?))
        }
        "nop" =>  {
            let arg = parts.get(1).ok_or("Invalid nop instruction.")?;
            Ok(Instruction::Nop(arg.trim_start_matches('+').parse()?))
        }
        _ => Err(From::from("Invalid instruction type."))
    }
}

fn parse_program(program: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let mut result = Vec::new();
    for line in program.split("\n") {
        match parse_instruction(line) {
            Ok(instr) => { result.push(instr); }
            Err(e) => { return Err(From::from(format!("Failed to parse instruction {}: {}", line, e))); }
        }
    }
    Ok(result)
}

#[derive(Debug)]
struct VM {
    pc: usize,
    acc: Word,
    mem: Vec<Instruction>,
}

impl VM {

    pub fn run(self: &mut Self) -> Result<Word, Box<dyn Error>> {
        let mut visited_pcs = HashSet::new();
        loop {
            if visited_pcs.contains(&self.pc) {
                return Err(From::from("Infinite loop."));
            } else if self.pc == self.mem.len() {
                return Ok(self.acc);
            }
            visited_pcs.insert(self.pc);
            let instr = self.mem[self.pc];
            match instr {
                Instruction::Acc(n) => {
                    self.acc += n;
                    self.pc += 1;
                }
                Instruction::Jmp(n) => {
                    let target = self.pc as Word + n;
                    if target < 0 || target as usize > self.mem.len() {
                        panic!("Jump took us to invalid pc.");
                    }
                    self.pc = target as usize;
                }
                Instruction::Nop(_) => {
                    self.pc += 1;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let instructions = parse_program(input.trim())?;
    for i in 0..instructions.len() {
        let mut vm = VM { pc: 0, acc: 0, mem: instructions.clone() };
        vm.mem[i] = match vm.mem[i] {
            Instruction::Acc(n) => Instruction::Acc(n),
            Instruction::Jmp(n) => Instruction::Nop(n),
            Instruction::Nop(n) => Instruction::Jmp(n)
        };
        match vm.run() {
            Ok(result) => println!("Result: {:?}", result),
            Err(_) => {}
        }
    }
    Ok(())
}
