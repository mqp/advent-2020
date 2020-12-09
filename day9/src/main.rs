use std::error::Error;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut nums = Vec::new();
    for line in input.trim().split_whitespace() {
        nums.push(line.parse::<i64>()?);
    }
    let target = 18272118;
    let mut cumulative_sum = 0;
    let mut sums_so_far = HashMap::new();
    for i in 0..nums.len() {
        sums_so_far.insert(cumulative_sum, i);
        cumulative_sum += nums[i];
        if let Some(left) = sums_so_far.get(&(cumulative_sum - target)) {
            let window = &nums[*left..(i + 1)];
            let min = window.iter().min().unwrap();
            let max = window.iter().max().unwrap();
            println!("Result: {}", min + max);
            break;
        }
    }
    Ok(())
}
