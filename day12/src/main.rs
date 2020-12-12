use std::error::Error;
use std::io::{self, Read};

fn parse_instruction(text: &str) -> Result<(char, i64), Box<dyn Error>> {
    let (left, right) = text.split_at(1);
    let arg = right.parse()?;
    Ok((left.chars().nth(0).unwrap(), arg))
}

fn rotate_cw(x: i64, y: i64, deg: i64) -> (i64, i64) {
    match deg % 360 {
        0 => (x, y),
        90 => (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        _ => { panic!("Non-right angle provided."); }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut instructions = Vec::new();
    for line in input.trim().split_whitespace() {
        instructions.push(parse_instruction(line)?);
    }
    let mut s_x = 0;
    let mut s_y = 0;
    let mut wp_x = 10;
    let mut wp_y = 1;
    for (instr, n) in instructions {
        match instr {
            'N' => { wp_y += n; }
            'S' => { wp_y -= n; }
            'E' => { wp_x += n; }
            'W' => { wp_x -= n; }
            'L' => {
                let (x, y) = rotate_cw(wp_x, wp_y, 360 - n);
                wp_x = x;
                wp_y = y;
            }
            'R' => {
                let (x, y) = rotate_cw(wp_x, wp_y, n);
                wp_x = x;
                wp_y = y;
            }
            'F' => {
                s_x += wp_x * n;
                s_y += wp_y * n;
            }
            _ => { panic!("Surprising instruction!"); }
        }
    }
    println!("Ending: {}, {}", s_x, s_y);
    Ok(())
}
