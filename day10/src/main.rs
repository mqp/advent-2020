use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;

fn count_arrangements(counts_from: &mut HashMap<usize, usize>, adapters: &[usize], curr_idx: usize) -> usize {
    match counts_from.get(&curr_idx) {
        Some(&ct) => ct,
        None => {
            let curr_adapter = adapters[curr_idx];
            let mut total = 0;
            if curr_idx == adapters.len() - 1 {
                total += 1;
            }
            if curr_idx + 1 < adapters.len() && adapters[curr_idx + 1] <= curr_adapter + 3 {
                total += count_arrangements(counts_from, adapters, curr_idx + 1);
            }
            if curr_idx + 2 < adapters.len() && adapters[curr_idx + 2] <= curr_adapter + 3 {
                total += count_arrangements(counts_from, adapters, curr_idx + 2);
            }
            if curr_idx + 3 < adapters.len() && adapters[curr_idx + 3] <= curr_adapter + 3 {
                total += count_arrangements(counts_from, adapters, curr_idx + 3);
            }
            counts_from.insert(curr_idx, total);
            return total;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut adapter_voltages = Vec::new();
    for line in input.trim().split("\n") {
        adapter_voltages.push(line.parse::<usize>()?);
    }
    adapter_voltages.sort();
    let mut cache = HashMap::new();
    let mut arrangements = count_arrangements(&mut cache, &adapter_voltages, 0);
    if adapter_voltages[1] <= 3 {
        arrangements += count_arrangements(&mut cache, &adapter_voltages, 1);
    }
    if adapter_voltages[2] <= 3 {
        arrangements += count_arrangements(&mut cache, &adapter_voltages, 2);
    }
    println!("Result: {}", arrangements);
    Ok(())
}
