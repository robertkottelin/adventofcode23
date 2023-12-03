fn sum_part_numbers(schematic: &[&str]) -> i32 {
    let mut sum = 0;
    let rows = schematic.len();
    let cols = schematic[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    for y in 0..rows {
        for x in 0..cols {
            if !visited[y][x] && schematic[y].chars().nth(x).unwrap().is_digit(10) {
                let number = extract_number(&schematic, &mut visited, y, x);
                if is_adjacent_to_symbol(&schematic, y, x) {
                    sum += number;
                }
            }
        }
    }

    sum
}

fn extract_number(schematic: &[&str], visited: &mut Vec<Vec<bool>>, y: usize, x: usize) -> i32 {
    let mut number = 0;
    let mut x_pos = x;

    while x_pos < schematic[y].len() && schematic[y].chars().nth(x_pos).unwrap().is_digit(10) {
        visited[y][x_pos] = true;
        number = number * 10 + schematic[y].chars().nth(x_pos).unwrap().to_digit(10).unwrap() as i32;
        x_pos += 1;
    }

    number
}

fn is_adjacent_to_symbol(schematic: &[&str], y: usize, x: usize) -> bool {
    let rows = schematic.len();
    let cols = schematic[0].len();

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 { continue; }

            let ny = y as i32 + dy;
            let nx = x as i32 + dx;

            if ny >= 0 && ny < rows as i32 && nx >= 0 && nx < cols as i32 {
                let adjacent_char = schematic[ny as usize].chars().nth(nx as usize).unwrap();
                if adjacent_char != '.' && !adjacent_char.is_digit(10) {
                    return true;
                }
            }
        }
    }

    false
}

fn main() {
    let schematic = [
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598.."
    ];

    println!("Sum of part numbers: {}", sum_part_numbers(&schematic));
}
