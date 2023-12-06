use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let path = Path::new("data.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut maps = HashMap::new();
    let mut current_map = String::new();
    let mut seeds = vec![];

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("seeds:") {
            seeds = line["seeds:".len()..].trim().split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();
        } else if line.ends_with("map:") {
            current_map = line;
            maps.insert(current_map.clone(), vec![]);
        } else if !line.is_empty() {
            if let Some(map) = maps.get_mut(&current_map) {
                let parts: Vec<i32> = line.split_whitespace()
                    .filter_map(|num| num.parse::<i32>().ok())
                    .collect();
                if parts.len() == 3 {
                    map.push((parts[0], parts[1], parts[2]));
                }
            }
        }
    }

    let mut lowest_location = i32::MAX;
    for seed in seeds {
        let mut current_value = seed;
        for map_name in ["seed-to-soil map:", "soil-to-fertilizer map:",
            "fertilizer-to-water map:", "water-to-light map:",
            "light-to-temperature map:", "temperature-to-humidity map:",
            "humidity-to-location map:"].iter() {
            if let Some(map) = maps.get(*map_name) {
                current_value = convert(current_value, map);
            }
        }
        lowest_location = lowest_location.min(current_value);
    }

    println!("Lowest location number: {}", lowest_location);
    Ok(())
}

fn convert(value: i32, map: &Vec<(i32, i32, i32)>) -> i32 {
    for &(dest_start, source_start, length) in map {
        if let Some(end) = source_start.checked_add(length) {
            if value >= source_start && value < end {
                return source_start.checked_add(value - source_start)
                    .and_then(|offset| dest_start.checked_add(offset))
                    .unwrap_or(value); // If overflow, return the original value
            }
        }
    }
    value
}
