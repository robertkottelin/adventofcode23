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
    // Define the function `parse_game` that takes a string slice `line` and returns an Option type with a tuple.
    // The tuple contains an integer (game ID) and a vector of integer triples (representing red, green, and blue cubes).

    let parts: Vec<&str> = line.split(": ").collect();
    // Split the input line at ": " to separate the game ID and the sets of cubes. Collect the results into a vector `parts`.

    if parts.len() != 2 {
        return None;
    }
    // Check if `parts` has exactly two elements (game ID and sets of cubes). If not, return None, indicating an invalid line format.

    let game_id = parts[0].trim_start_matches("Game ").parse::<i32>().ok()?;
    // Extract the game ID from the first part, remove the "Game " prefix, parse it as an integer, and handle errors using `?` which returns None if parsing fails.

    let sets_str = parts[1].split("; ");
    // Split the second part of `parts` at each "; " to separate individual sets of cubes.

    let mut sets = Vec::new();
    // Create a mutable vector `sets` to store the sets of cubes.

    for set_str in sets_str {
        // Iterate over each set of cubes represented as a string.

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        // Initialize counters for red, green, and blue cubes to 0.

        for cube_str in set_str.split(", ") {
            // Split each set into individual color counts separated by ", " and iterate over them.

            let cube_parts: Vec<&str> = cube_str.split_whitespace().collect();
            // Split each color count at whitespace and collect into a vector `cube_parts`.

            if cube_parts.len() != 2 {
                return None;
            }
            // Check if `cube_parts` has exactly two elements (count and color). If not, return None, indicating an invalid format.

            let count = cube_parts[0].parse::<i32>().ok()?;
            // Parse the first element of `cube_parts` as an integer (the count of cubes) and handle errors with `?`.

            match cube_parts[1] {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => return None,
            }
            // Match the second element (color) and assign the parsed count to the corresponding color variable. Return None if the color is not recognized.
        }

        sets.push((red, green, blue));
        // Push the tuple of counts (red, green, blue) for the current set into the `sets` vector.
    }

    Some((game_id, sets))
    // Return Some with the tuple containing the game ID and the vector of sets of cubes.
}
