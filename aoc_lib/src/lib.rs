use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path; // Import Path for path manipulation

pub fn read_single_string(day: u8) -> io::Result<String> {
    // Construct the filename based on the day number
    // Assumes input text files are in a directory named "inputs" one level up from aoc_lib
    let filename = format!("inputs/{:02}.txt", day);

    // Read the entire content of the file into a String.
    fs::read_to_string(&Path::new(&filename))
}

pub fn read_multiple_strings(day: u8) -> io::Result<Vec<String>> {
    // Construct the filename based on the day number
    // Assumes input text files are in a directory named "inputs" one level up from aoc_lib
    let filename = format!("inputs/{:02}.txt", day);
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        lines.push(line);
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // TODO: Add tests here for read_input or other aoc_lib functions
        // For example, a test that checks if reading a non-existent file panics (as expected!)
        assert!(true); // Placeholder test
    }
}
