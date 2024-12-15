use regex::Regex;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("input.real")?;

    let mul_regex = Regex::new(r"mul\(\d+,\d+\)")?;
    let matches = mul_regex
        .find_iter(&file)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();

    let numbers_regex = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let numbers: Vec<(i32, i32)> = matches
        .iter()
        .filter_map(|&m| {
            numbers_regex.captures(m).map(|caps| {
                let a = caps[1].parse::<i32>().unwrap();
                let b = caps[2].parse::<i32>().unwrap();
                (a, b)
            })
        })
        .collect();

    let res = numbers.iter().fold(0, |acc, x| acc + (x.0 * x.1));
    println!("{res}");
    Ok(())
}
