use std::error::Error;
use std::io::{self, Read};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum SeatLocation {
    Floor,
    Empty,
    Occupied
}

type Grid = Vec<Vec<SeatLocation>>;

fn print_grid(grid: &Grid) {
    for row in grid {
        let mut line = String::new();
        for loc in row {
            line.push(match loc {
                SeatLocation::Floor => '.',
                SeatLocation::Empty => 'L',
                SeatLocation::Occupied => '#'
            });
        }
        println!("{}", line);
    }
}

fn parse_grid(text: &str) -> Grid {
    let mut result = Vec::new();
    for line in text.split_whitespace() {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(match ch {
                'L' => SeatLocation::Empty,
                '#' => SeatLocation::Occupied,
                _ => SeatLocation::Floor
            });
        }
        result.push(row);
    }
    result
}

fn count_occ(grid: &Grid) -> usize {
    let mut total = 0;
    for row in grid {
        for loc in row {
            if *loc == SeatLocation::Occupied {
                total += 1;
            }
        }
    }
    total
}

fn find_visible_occ(grid: &Grid, x: i64, y: i64, dx: i64, dy: i64) -> bool {
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;
    let mut gx = x + dx;
    let mut gy = y + dy;
    while gx >= 0 && gx < width && gy >= 0 && gy < height {
        if grid[gy as usize][gx as usize] == SeatLocation::Occupied {
            return true;
        }
        if grid[gy as usize][gx as usize] == SeatLocation::Empty {
            return false;
        }
        gx += dx;
        gy += dy;
    }
    false
}

fn count_visible_occ(grid: &Grid, x: i64, y: i64) -> usize {
    let mut total = 0;
    for dy in -1..2 {
        for dx in -1..2 {
            if !(dx == 0 && dy == 0) {
                if find_visible_occ(grid, x, y, dx, dy) {
                    total += 1;
                }
            }
        }
    }
    total
}

fn process_grid(input_grid: &Grid, output_grid: &mut Grid) -> bool {
    let mut changed = false;
    for y in 0..input_grid.len() {
        let input_row = &input_grid[y];
        let output_row = &mut output_grid[y];
        for x in 0..input_row.len() {
            let input_loc = input_row[x];
            match input_loc {
                SeatLocation::Floor => {}
                SeatLocation::Empty => {
                    if count_visible_occ(input_grid, x as i64, y as i64) == 0 {
                        output_row[x] = SeatLocation::Occupied;
                        changed = true;
                    }

                }
                SeatLocation::Occupied => {
                    if count_visible_occ(input_grid, x as i64, y as i64) >= 5 {
                        output_row[x] = SeatLocation::Empty;
                        changed = true;
                    }
                }
            }
        }
    }
    changed
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut grid = parse_grid(input.trim());
    let mut output = grid.clone();
    loop {
        output = grid.clone();
        if !process_grid(&grid, &mut output) {
            break;
        }
        print_grid(&output);
        println!();
       grid = output.clone();
    }
    println!("Result: {}", count_occ(&output));
    Ok(())
}
