
pub fn print_grid<T: std::fmt::Display>(grid: &[Vec<T>]) {
    for row in grid {
        for item in row {
            print!("{}\t", item);
        }
        println!();
    }
}
