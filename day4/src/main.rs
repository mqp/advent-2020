use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use regex::Regex;

/*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
*/

fn is_valid(items: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let byr = items.get("byr").ok_or("No byr provided.")?.parse::<u32>()?;
    if byr < 1920 || byr > 2002 {
        return Err(From::from("byr out of range."));
    }
    let iyr = items.get("iyr").ok_or("No iyr provided.")?.parse::<u32>()?;
    if iyr < 2010 || iyr > 2020 {
        return Err(From::from("iyr out of range."));
    }
    let eyr = items.get("eyr").ok_or("No eyr provided.")?.parse::<u32>()?;
    if eyr < 2020 || eyr > 2030 {
        return Err(From::from("eyr out of range."));
    }
    let hgt = items.get("hgt").ok_or("No hgt provided.")?;
    let hgt_val = hgt[..hgt.len()-2].parse::<u32>()?;
    let hgt_units = &hgt[hgt.len()-2..];
    match hgt_units {
        "cm" => if hgt_val < 150 || hgt_val > 193 {
            return Err(From::from("hgt value out of range."));
        }
        "in" => if hgt_val < 59 || hgt_val > 76 {
            return Err(From::from("hgt value out of range."));
        }
        _ => return Err(From::from("hgt units out of range."))
    }
    let hcl = items.get("hcl").ok_or("No hcl provided.")?;
    let color_re = Regex::new(r"^\#([0-9a-f]){6}$").unwrap();
    if !color_re.is_match(&hcl) {
        return Err(From::from("Invalid hcl."));
    }
    let ecl = items.get("ecl").ok_or("No ecl provided.")?;
    if ecl != "amb" && ecl != "blu" && ecl != "brn" && ecl != "gry" &&
        ecl != "grn" && ecl != "hzl" && ecl != "oth" {
        return Err(From::from("Invalid ecl."));
    }
    let pid = items.get("pid").ok_or("No pid provided.")?;
    if pid.len() != 9 {
        return Err(From::from("pid out of range."));
    }
    pid.parse::<u32>()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let passports = input.split("\n\n").collect::<Vec<_>>();
    let mut valid = 0;
    let mut invalid = 0;
    for passport in passports {
        let mut items = HashMap::new();
        for item in passport.split_whitespace() {
            let parts = item.split(':').collect::<Vec<_>>();
            let name = parts.get(0).expect("No field name provided.");
            let value = parts.get(1).expect("No field name provided.");
            items.insert(name.to_string(), value.to_string());
        }
        if let Err(e) = is_valid(&items) {
            invalid += 1;
            println!("{} {:?}", e, items);
        } else {
            valid += 1;
        }
    }
    println!("Result is: {}/{}", valid, invalid + valid);
    Ok(())
}
