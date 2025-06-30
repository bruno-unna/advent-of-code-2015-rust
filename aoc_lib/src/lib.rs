use std::fs; // Import the file system module
use std::path::Path; // Import Path for path manipulation

pub fn read_input(day: u8) -> String {
    // Construct the filename based on the day number
    // Assumes input text files are in a directory named "inputs" one level up from aoc_lib
    let filename = format!("inputs/{:02}.txt", day);

    // Read the entire content of the file into a String.
    // `.expect()` is used here for simplicity in Advent of Code;
    // in production code, you'd typically handle the Result<String, io::Error> more gracefully.
    fs::read_to_string(&Path::new(&filename))
        .expect(&format!("Could not read input text file for Day {}. Make sure '{}' exists in the 'inputs' directory.", day, filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // TODO: Add tests here for read_input or other aoc_lib functions
        // For example, a test that checks if reading a non-existent file panics (as per expect)
        assert!(true); // Placeholder test
    }
}
