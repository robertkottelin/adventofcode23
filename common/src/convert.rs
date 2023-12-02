
pub fn lines_to_numbers(lines: Vec<String>) -> Result<Vec<i32>, std::num::ParseIntError> {
    lines.into_iter().map(|line| line.parse()).collect()
}
