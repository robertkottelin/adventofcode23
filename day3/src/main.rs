use std::{fs, collections::HashMap};

fn main() {
    let input = read_and_parse_input("data.txt");
    let (part_numbers, gears) = find_numbers_and_neighbors(&input);
    let part1 = sum_part_numbers(&part_numbers);
    let part2 = calculate_gear_powers(gears);

    dbg!(part1);
    dbg!(part2);
}

fn read_and_parse_input(filename: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(filename).expect("Failed to read file");
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_numbers_and_neighbors(input: &[Vec<char>]) -> (Vec<u32>, HashMap<(usize, usize), Vec<u32>>) {
    let mut part_numbers = vec![];
    let mut gears = HashMap::<(usize, usize), Vec<u32>>::new();

    for (row, line) in input.iter().enumerate() {
        let mut col1 = 0;
        while col1 < line.len() {
            if let Some((n, col2)) = find_next_number(line, col1) {
                if let Some(neighbors) = process_neighbors(input, row, col1, col2, n) {
                    gears.extend(neighbors);
                }
                part_numbers.push(n);
                col1 = col2;
            } else {
                break;
            }
        }
    }

    (part_numbers, gears)
}

fn find_next_number(line: &[char], start_col: usize) -> Option<(u32, usize)> {
    let mut col = start_col;
    while col < line.len() && !line[col].is_numeric() {
        col += 1;
    }

    if col < line.len() {
        let end_col = line.iter().skip(col).take_while(|c| c.is_numeric()).count() + col;
        let number: u32 = line[col..end_col].iter().collect::<String>().parse().unwrap();
        Some((number, end_col))
    } else {
        None
    }
}

fn process_neighbors(input: &[Vec<char>], row: usize, start_col: usize, end_col: usize, number: u32) -> Option<HashMap<(usize, usize), Vec<u32>>> {
    let mut neighbors = HashMap::new();
    let start_row = if row > 0 { row - 1 } else { 0 };
    let end_row = (row + 2).min(input.len());
    let start_col = if start_col > 0 { start_col - 1 } else { 0 };
    let end_col = (end_col + 1).min(input[row].len());

    for i in start_row..end_row {
        for j in start_col..end_col {
            if input[i][j] != '.' && !input[i][j].is_numeric() {
                if input[i][j] == '*' {
                    neighbors.entry((i, j)).or_insert_with(Vec::new).push(number);
                }
                return Some(neighbors);
            }
        }
    }

    None
}

fn sum_part_numbers(part_numbers: &[u32]) -> u32 {
    part_numbers.iter().sum()
}

fn calculate_gear_powers(gears: HashMap<(usize, usize), Vec<u32>>) -> u32 {
    gears.into_values().filter(|g| g.len() == 2).map(|g| g.into_iter().product::<u32>()).sum()
}
