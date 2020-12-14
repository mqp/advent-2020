use std::error::Error;
use std::io::{self, Read};

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn find_total(moduli: &[i64], remainders: &[i64]) -> i64 {
    let mut big_m = 1;
    for m in moduli {
        big_m *= m;
    }
    let mut total = 0;
    for i in 0..remainders.len() {
        let b = big_m / moduli[i];
        total += remainders[i] * b * mod_pow(b, moduli[i]-2, moduli[i]);
        total %= big_m;
    }
    total
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let lines = input.trim().split_whitespace().collect::<Vec<_>>();
    let mut moduli = Vec::new();
    let mut remainders = Vec::new();
    for (i, id) in lines[1].split(',').enumerate() {
        println!("{} {}", i, id);
        if id != "x" {
            let m = id.parse()?;
            moduli.push(m);
            remainders.push(m as i64 - i as i64);
        }
    }
    println!("Result: {}", find_total(&moduli, &remainders));
    Ok(())
}
