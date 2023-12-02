extern crate common;
use common::file::read_lines;
use std::io;

// Part 1
fn main() -> io::Result<()> {
    let lines = read_lines("data.txt")?;

    let sum = lines.iter()
        .filter_map(|line| {
            let chars: Vec<char> = line.chars().collect();
            if let (Some(first_digit), Some(last_digit)) = (chars.iter().find(|&&c| c.is_digit(10)), chars.iter().rev().find(|&&c| c.is_digit(10))) {
                let value = first_digit.to_string() + &last_digit.to_string();
                value.parse::<i32>().ok()
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("Total sum of calibration values: {}", sum);

    Ok(())
}

// Part 2
// ??