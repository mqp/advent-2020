use std::error::Error;
use std::io::{self, Read};
use std::collections::HashSet;

type Grid = HashSet<(i64, i64, i64, i64)>;

fn parse_grid(text: &str) -> Grid {
    let mut result = HashSet::new();
    for (y, line) in text.split_whitespace().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                result.insert((x as i64, y as i64, 0 as i64, 0 as i64));
            }
        }
    }
    println!("Start: {:?}", result);
    result
}

fn count_neighbors(grid: &Grid, x: i64, y: i64, z: i64, w: i64) -> usize {
    let mut total = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            for dz in -1..2 {
                for dw in -1..2 {
                    if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                        if grid.contains(&(x + dx, y + dy, z + dz, w + dw)) {
                            total += 1;
                        }
                    }
                }
            }
        }
    }
    total
}

fn should_activate(grid: &Grid, x: i64, y: i64, z: i64, w: i64) -> bool {
    let ct = count_neighbors(grid, x, y, z, w);
    if grid.contains(&(x, y, z, w)) {
        ct == 2 || ct == 3
    } else {
        ct == 3
    }
}

fn process_grid(input_grid: &Grid, output_grid: &mut Grid) {
    if input_grid.len() == 0 {
        *output_grid = HashSet::new();
        return;
    }
    let (min_x, _, _, _) = input_grid.iter().min_by_key(|&(x, _y, _z, _w)| x).unwrap();
    let (max_x, _, _, _) = input_grid.iter().max_by_key(|&(x, _y, _z, _w)| x).unwrap();
    let (_, min_y, _, _) = input_grid.iter().min_by_key(|&(_x, y, _z, _w)| y).unwrap();
    let (_, max_y, _, _) = input_grid.iter().max_by_key(|&(_x, y, _z, _w)| y).unwrap();
    let (_, _, min_z, _) = input_grid.iter().min_by_key(|&(_x, _y, z, _w)| z).unwrap();
    let (_, _, max_z, _) = input_grid.iter().max_by_key(|&(_x, _y, z, _w)| z).unwrap();
    let (_, _, _, min_w) = input_grid.iter().min_by_key(|&(_x, _y, _z, w)| w).unwrap();
    let (_, _, _, max_w) = input_grid.iter().max_by_key(|&(_x, _y, _z, w)| w).unwrap();
    for x in (min_x - 1)..(max_x + 2) {
        for y in (min_y - 1)..(max_y + 2) {
            for z in (min_z - 1)..(max_z + 2) {
                for w in (min_w - 1)..(max_w + 2) {
                    if should_activate(input_grid, x, y, z, w) {
                        output_grid.insert((x, y, z, w));
                    } else {
                        output_grid.remove(&(x, y, z, w));
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut grid = parse_grid(input.trim());
    let mut output = grid.clone();
    for _ in 0..6 {
        output = grid.clone();
        process_grid(&grid, &mut output);
        grid = output.clone();
    }
    println!("Result: {}", output.len());
    Ok(())
}
