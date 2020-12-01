use std::fs::File;
use std::error::Error;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("input")?;
    let reader = BufReader::new(input);
    let mut nums = HashSet::<i32>::new();
    for line in reader.lines() {
        nums.insert(line?.parse()?);
    }

    let total = 2020;
    for n0 in &nums {
        for n1 in &nums {
            let candidate = total - n0 - n1;
            if nums.contains(&candidate) {
                println!("Result is: {}", n0 * n1 * candidate);
                return Ok(());
            }
        }
    }
    Ok(())
}
