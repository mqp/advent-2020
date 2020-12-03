use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct SlopeMap {
    items: Vec<Vec<bool>>
}

impl SlopeMap {
    pub fn new() -> SlopeMap {
        Self { items: Vec::new() }
    }
    pub fn calculate_trees(&self, slope_x: usize, slope_y: usize) -> usize {
        let height = self.items.len();
        let mut trees = 0;
        let mut curr_x = slope_x;
        let mut curr_y = slope_y;
        while curr_y < height {
            let row = self.items.get(curr_y).unwrap();
            if row.get(curr_x % row.len()) == Some(&true) {
                trees += 1;
            }
            curr_x += slope_x;
            curr_y += slope_y;
        }
        trees
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("input")?;
    let reader = BufReader::new(input);
    let mut map = SlopeMap::new();
    for line in reader.lines() {
        map.items.push(line?.chars().map(|ch| ch == '#').collect::<Vec<_>>());
    }
    let t0 = map.calculate_trees(1, 1);
    let t1 = map.calculate_trees(3, 1);
    let t2 = map.calculate_trees(5, 1);
    let t3 = map.calculate_trees(7, 1);
    let t4 = map.calculate_trees(1, 2);
    println!("Result is: {} * {} * {} * {} * {}: {}", t0, t1, t2, t3, t4, t0 * t1 * t2 * t3 * t4);
    Ok(())
}
