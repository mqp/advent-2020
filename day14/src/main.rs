use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;

type Word = usize;
type Memory = HashMap<usize, Word>;

fn parse_mask(text: &str) -> &str {
    text.trim_start_matches("mask = ")
}

fn parse_assignment(text: &str) -> Result<(usize, Word), Box<dyn Error>> {
    let parts = text.split(" = ").collect::<Vec<_>>();
    let dest = parts.get(0).ok_or("Invalid assignment format")?;
    let addr = dest.trim_start_matches("mem[").trim_end_matches("]").parse()?;
    let val = parts.get(1).ok_or("Invalid assignment format")?.parse()?;
    Ok((addr, val))
}

fn apply_mask(mut arg: Word, mask: &str) -> Result<Word, Box<dyn Error>> {
    arg |= Word::from_str_radix(&mask.replace('X', "0"), 2)?;
    arg &= Word::from_str_radix(&mask.replace('X', "1"), 2)?;
    Ok(arg)
}

fn apply_mask_all(arg: Word, mask: &str) -> Vec<Word> {
    let mut result = Vec::new();
    match mask.chars().last() {
        None => { result.push(0); }
        Some(bit) => {
            for m in apply_mask_all(arg / 2, &mask[0..mask.len()-1]) {
                match bit {
                    '0' => { result.push(2 * m + arg % 2); }
                    '1' => { result.push(2 * m + 1); }
                    _ => {
                        result.push(2 * m + 0);
                        result.push(2 * m + 1);
                    }
                }
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut mem = Memory::new();
    let mut mask = String::new();
    for line in input.trim().split("\n") {
        if line.starts_with("mem") {
            let (addr, val) = parse_assignment(line)?;
            for addr in apply_mask_all(addr, &mask) {
                mem.insert(addr as usize, val);
            }
        } else if line.starts_with("mask") {
            mask = parse_mask(line).to_string();
        } else {
            panic!("Invalid instruction.");
        }
    }
    println!("Result: {}", mem.values().sum::<Word>());
    Ok(())
}
