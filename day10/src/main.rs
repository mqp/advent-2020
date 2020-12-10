use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;

fn count_chains(cache: &mut HashMap<usize, usize>, adapters: &[usize], i: usize) -> usize {
    match cache.get(&i) {
        Some(&ct) => ct,
        None => {
            let voltage = adapters[i];
            let mut total = 0;
            if i == adapters.len() - 1 {
                total += 1;
            }
            if i + 1 < adapters.len() && adapters[i + 1] <= voltage + 3 {
                total += count_chains(cache, adapters, i + 1);
            }
            if i + 2 < adapters.len() && adapters[i + 2] <= voltage + 3 {
                total += count_chains(cache, adapters, i + 2);
            }
            if i + 3 < adapters.len() && adapters[i + 3] <= voltage + 3 {
                total += count_chains(cache, adapters, i + 3);
            }
            cache.insert(i, total);
            total
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut adapters = vec!(0);
    for line in input.trim().split("\n") {
        adapters.push(line.parse()?);
    }
    adapters.sort();
    let mut cache = HashMap::new();
    println!("Result: {}", count_chains(&mut cache, &adapters, 0));
    Ok(())
}
