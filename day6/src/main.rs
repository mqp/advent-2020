use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let groups = input.trim().split("\n\n").collect::<Vec<_>>();
    let mut total_counts = 0;
    for group in groups {
        let people = group.trim().split("\n").collect::<Vec<_>>();
        let mut group_yeses = "abcdefghijklmnopqrstuvwxyz".chars().collect::<HashSet<_>>();
        for person in people {
            group_yeses.retain(|&ch| person.contains(ch));
        }
        println!("Group yeses: {:?}", group_yeses);
        total_counts += group_yeses.len();
    }
    println!("Result: {}", total_counts);
    Ok(())
}
