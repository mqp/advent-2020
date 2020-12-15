use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;

const CT: usize = 30000000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut numbers = Vec::with_capacity(CT);
    let mut timestamps_by_n = HashMap::with_capacity(CT);
    for (i, x) in input.trim().split(',').enumerate() {
        let n = x.parse::<usize>()?;
        timestamps_by_n.insert(n, i);
        numbers.push(n);
    }
    timestamps_by_n.remove(numbers.last().unwrap());
    for i in numbers.len()..CT {
        let prev = *numbers.last().unwrap();
        if let Some(last_ts) = timestamps_by_n.get(&prev) {
            numbers.push(i - 1 - last_ts);
        } else {
            numbers.push(0);
        }
        timestamps_by_n.insert(prev, i - 1);
    }
    println!("Result: {:?}", numbers.last());
    Ok(())
}
