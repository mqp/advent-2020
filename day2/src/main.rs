use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

fn is_valid(policy_ch: char, policy_i: usize, policy_j: usize, password: &str) -> bool {
    let i_match = password.chars().nth(policy_i - 1).expect("Invalid password i-index.") == policy_ch;
    let j_match = password.chars().nth(policy_j - 1).expect("Invalid password j-index.") == policy_ch;
    (i_match && !j_match) || (!i_match && j_match)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("input")?;
    let reader = BufReader::new(input);
    let mut valid: usize = 0;
    for line in reader.lines() {
        let whole = line?;
        let parts = whole.split_whitespace().collect::<Vec<_>>();
        let policy_indices = parts.get(0).expect("Error parsing policy range.");
        let index_parts = policy_indices.split('-').collect::<Vec<_>>();
        let policy_i = index_parts.get(0).expect("Error parsing policy range minimum.").parse()?;
        let policy_j = index_parts.get(1).expect("Error parsing policy range maximum.").parse()?;
        let character = parts.get(1).expect("Error parsing policy character.").chars().nth(0).unwrap();
        let password = parts.get(2).expect("Error parsing password.");
        if is_valid(character, policy_i, policy_j, password) {
            valid += 1;
        }
    }
    println!("Result is: {}", valid);
    Ok(())
}
