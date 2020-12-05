use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

fn find_index(partitions: &[bool]) -> usize {
    let mut min = 0;
    let mut max = usize::pow(2, partitions.len() as u32) - 1;
    for p in partitions {
        match p {
            true  => { min += (max - min + 1) / 2; }
            false => { max -= (max - min + 1) / 2; }
        }
    }
    assert!(min == max);
    min
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let passes = input.trim().split("\n").collect::<Vec<_>>();
    let mut seats_taken = HashSet::new();
    for pass in passes {
        let row_partitions = pass[..7].chars().map(|ch| ch == 'B').collect::<Vec<_>>();
        let col_partitions = pass[7..10].chars().map(|ch| ch == 'R').collect::<Vec<_>>();
        let row_index = find_index(&row_partitions);
        let col_index = find_index(&col_partitions);
        seats_taken.insert((row_index, col_index));
    }
    for row in 0..128 {
        for col in 0..8 {
            if !seats_taken.contains(&(row, col)) {
                println!("Empty seat: {} {} {}", row, col, row * 8 + col);
            }
        }
    }
    Ok(())
}
