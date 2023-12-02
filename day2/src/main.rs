use std::fs::File;
use std::io::{self, BufRead};

// // Part 1
// fn main() -> io::Result<()> {
//     let path = "data.txt"; // Replace with the path to your file
//     let file = File::open(path)?;
//     let reader = io::BufReader::new(file);

//     let max_cubes = (12, 13, 14); // (red, green, blue)
//     let mut possible_games_sum = 0;

//     for line in reader.lines() {
//         let line = line?;
//         if let Some((game_id, sets)) = parse_game(&line) {
//             if sets.iter().all(|&(red, green, blue)| {
//                 red <= max_cubes.0 && green <= max_cubes.1 && blue <= max_cubes.2
//             }) {
//                 possible_games_sum += game_id;
//             }
//         }
//     }

//     println!("Sum of IDs of possible games: {}", possible_games_sum);
//     Ok(())
// }

// Part 2
fn main() -> io::Result<()> {
    let path = "data.txt"; 
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut sum_of_powers = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some((_game_id, sets)) = parse_game(&line) {
            let (min_red, min_green, min_blue) = sets.iter().fold((0, 0, 0), |(max_red, max_green, max_blue), &(red, green, blue)| {
                (max_red.max(red), max_green.max(green), max_blue.max(blue))
            });

            sum_of_powers += min_red * min_green * min_blue;
        }
    }

    println!("Sum of the powers of the minimum sets: {}", sum_of_powers);
    Ok(())
}

fn parse_game(line: &str) -> Option<(i32, Vec<(i32, i32, i32)>)> {

    let parts: Vec<&str> = line.split(": ").collect();

    if parts.len() != 2 {
        return None;
    }

    let game_id = parts[0].trim_start_matches("Game ").parse::<i32>().ok()?;

    let sets_str = parts[1].split("; ");

    let mut sets = Vec::new();

    for set_str in sets_str {

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cube_str in set_str.split(", ") {

            let cube_parts: Vec<&str> = cube_str.split_whitespace().collect();

            if cube_parts.len() != 2 {
                return None;
            }

            let count = cube_parts[0].parse::<i32>().ok()?;

            match cube_parts[1] {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => return None,
            }
        }

        sets.push((red, green, blue));
    }

    Some((game_id, sets))
}
