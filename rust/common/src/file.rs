use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    // Attempt to open the file with the given filename. The `?` operator is used to return the error if the operation fails.
    let file = File::open(filename)?;
    // Create a buffered reader for the file, which provides efficient reading of lines.
    let reader = BufReader::new(file);
    // Read lines from the buffered reader into a vector, and return it. If there's an error, the `?` operator will return it from the function.
    reader.lines().collect()
}