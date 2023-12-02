

pub fn split_string_by_delimiter(input: &str, delimiter: char) -> Vec<&str> {
    input.split(delimiter).collect()
}
