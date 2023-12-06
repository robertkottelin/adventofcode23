use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("data.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let cards: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut total_cards = 0;
    let mut queue: Vec<usize> = (0..cards.len()).collect();

    while let Some(index) = queue.pop() {
        total_cards += 1;
        let card = &cards[index];
        let matches = calculate_matches(card).unwrap_or(0);

        // Generate new cards based on the number of matches
        for i in 1..=matches {
            if index + i < cards.len() {
                queue.push(index + i);
            }
        }
    }

    println!("Total scratchcards: {}", total_cards);
    Ok(())
}

fn calculate_matches(card: &str) -> Result<usize, &'static str> {
    let parts: Vec<&str> = card.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid card format");
    }

    let number_parts: Vec<&str> = parts[1].split('|').collect();
    if number_parts.len() != 2 {
        return Err("Invalid number format");
    }

    let winning_numbers = parse_numbers(number_parts[0])?;
    let elf_numbers = parse_numbers(number_parts[1])?;

    Ok(elf_numbers.iter().filter(|&n| winning_numbers.contains(n)).count())
}

fn parse_numbers(s: &str) -> Result<Vec<i32>, &'static str> {
    s.trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().map_err(|_| "Invalid number"))
        .collect()
}
