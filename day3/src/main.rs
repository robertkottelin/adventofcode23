use std::collections::HashSet;

fn sum_part_numbers(schematic: &[String]) -> i32 {
    let mut sum = 0;
    let mut counted = HashSet::new();
    let rows = schematic.len();
    let cols = schematic[0].len();

    let is_symbol = |c: char| !c.is_digit(10) && c != '.';

    for r in 0..rows {
        for c in 0..cols {
            // Check if the current character is a digit and not already counted
            if schematic[r].as_bytes()[c].is_ascii_digit() && !counted.contains(&(r, c)) {
                let mut adjacent_to_symbol = false;

                // Check all adjacent cells for a symbol
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        let new_r = r as i32 + dr;
                        let new_c = c as i32 + dc;

                        if new_r >= 0 && new_r < rows as i32 && new_c >= 0 && new_c < cols as i32 {
                            if is_symbol(schematic[new_r as usize].as_bytes()[new_c as usize] as char) {
                                adjacent_to_symbol = true;
                            }
                        }
                    }
                }

                if adjacent_to_symbol {
                    // If adjacent to a symbol, parse the entire number
                    let mut num_str = String::new();
                    let mut cc = c;

                    while cc < cols && schematic[r].as_bytes()[cc].is_ascii_digit() {
                        num_str.push(schematic[r].as_bytes()[cc] as char);
                        counted.insert((r, cc)); // Mark this digit as counted
                        cc += 1;
                    }

                    if let Ok(num) = num_str.parse::<i32>() {
                        sum += num;
                    }
                }
            }
        }
    }

    sum
}

fn main() {
    let schematic = vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    ];

    let result = sum_part_numbers(&schematic);
    println!("Sum of part numbers: {}", result);
}
