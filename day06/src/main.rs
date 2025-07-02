//! Advent of Code 2015 - Day 6: "Probably a Fire Hazard"
//!
//! This module implements the solution for Day 6, which involves controlling a 1000x1000 grid of lights
//! based on instructions parsed from input. Part 1 treats lights as binary (on/off),
//! while Part 2 treats them as having variable brightness.
//!
//! The solution uses a global static `FIELD` (a 2D vector) protected by a `Mutex`
//! to simulate the grid of lights, allowing mutable access across function calls.
//! Regular expressions are used to parse the instructions efficiently.

use aoc_lib::read_multiple_strings;

use regex::Regex;
use std::sync::{LazyLock, Mutex}; // LazyLock for lazy static initialisation, Mutex for thread-safe mutable static // Used for parsing instruction strings

/// Main entry point of the Day 6 solution.
///
/// Reads the input lines, converts them to string slices, and then
/// prints the results for Part 1 (counting lights on) and
/// Part 2 (summing total brightness).
fn main() {
    let lines_result = read_multiple_strings(6);

    // Use `if let Ok(...)` for better error handling than `if .is_ok() .unwrap()`
    if let Ok(owned_lines_vec) = lines_result {
        let input_line_slices: Vec<&str> = owned_lines_vec
            .iter()
            .map(|s| s.as_str()) // Convert `&String` to `&str`
            .collect(); // Collect these `&str` into a new `Vec<&str>`

        println!("Day 6 Part 1: {}", solve_part1(&input_line_slices));
        println!("Day 6 Part 2: {}", solve_part2(&input_line_slices));
    } else {
        eprintln!("Error reading input: {:?}", lines_result.err());
    }
}

/// Constant for the number of rows in the grid.
const ROWS: usize = 1000;
/// Constant for the number of columns in the grid.
const COLS: usize = 1000;

/// A global static representation of the light grid.
///
/// It's a `LazyLock` for lazy initialisation, and wrapped in a `Mutex`
/// to allow mutable access across different functions and potentially threads.
/// It stores `i32` values to represent brightness for Part 2, and can represent
/// on/off states (1/0) for Part 1.
static FIELD: LazyLock<Mutex<Vec<Vec<i32>>>> =
    LazyLock::new(|| Mutex::new(vec![vec![0; COLS]; ROWS]));

/// Represents the type of command in an instruction.
#[derive(Debug)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

/// Represents a coordinate point in the 2D grid.
#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
}

/// Represents a single instruction for controlling lights.
///
/// Contains the command type and the two corner points defining the affected rectangular region.
#[derive(Debug)]
struct Instruction {
    cmd: Command,
    corners: (Point, Point),
}

/// A `LazyLock` for the regular expression used to parse instruction strings.
///
/// It captures the command (e.g., "turn on") and the four coordinate numbers.
/// The pattern is `^(command) (x0),(y0) through (x1),(y1)$`.
static CMD_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap()
});

impl Instruction {
    /// Creates a new `Instruction` from a raw string slice.
    ///
    /// Parses the string using `CMD_RE` and extracts the command and coordinates.
    /// Returns `Ok(Instruction)` if parsing is successful, or `Err(String)` with an error message
    /// if the string format is incorrect or values cannot be parsed.
    ///
    /// # Arguments
    ///
    /// * `instruction_string` - The raw string representation of the instruction.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Instruction`) or failure (`String` error message).
    fn new(instruction_string: &str) -> Result<Instruction, String> {
        // Attempt to capture parts of the string using the regex
        if let Some(captures) = CMD_RE.captures(instruction_string) {
            // Match the captured command string (group 1) to determine the Command enum variant
            let cmd = match &captures[1] {
                "turn on" => Command::TurnOn,
                "turn off" => Command::TurnOff,
                "toggle" => Command::Toggle,
                // This arm handles cases where the regex might capture something unexpected in group 1.
                _ => {
                    return Err(format!(
                        "unrecognized command in regex capture: '{}'",
                        &captures[1]
                    ));
                }
            };

            // Attempt to parse all four coordinate numbers (groups 2 through 5) into i16.
            // The `if let (Ok(...), ...)` syntax allows chaining multiple Result unwrap attempts.
            if let (Ok(x0), Ok(y0), Ok(x1), Ok(y1)) = (
                captures[2].parse::<i16>(),
                captures[3].parse::<i16>(),
                captures[4].parse::<i16>(),
                captures[5].parse::<i16>(),
            ) {
                // If all parsing is successful, return the new Instruction
                return Ok(Instruction {
                    cmd, // Shorthand for cmd: cmd
                    corners: (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }),
                });
            } else {
                // If any coordinate parsing fails, return an error
                return Err(format!(
                    "couldn't parse coordinates at line\n\t{}",
                    instruction_string
                ));
            }
        } else {
            // If the regex doesn't match the instruction string at all, return an error
            return Err(format!(
                "line\n\t{}\n couldn't be parsed as an instruction",
                instruction_string
            ));
        }
    }

    /// Executes the instruction according to Day 6, Part 1 rules.
    ///
    /// Modifies the global `FIELD` grid.
    /// - `TurnOn`: Sets lights in the region to 1 (on).
    /// - `TurnOff`: Sets lights in the region to 0 (off).
    /// - `Toggle`: Flips lights in the region (0 becomes 1, 1 becomes 0).
    ///
    /// # Panics
    /// Panics if the `FIELD` mutex cannot be locked.
    fn execute_wrong(&self) {
        println!(
            "executing instruction {:?} in range {:?} through {:?}",
            self.cmd, self.corners.0, self.corners.1
        );

        // Acquire a mutable lock on the global FIELD to modify it
        let mut field_guard = FIELD.lock().unwrap();

        // Iterate over the rectangular region defined by the instruction's corners
        for row_i16 in self.corners.0.y..=self.corners.1.y {
            for col_i16 in self.corners.0.x..=self.corners.1.x {
                // Cast i16 coordinates to usize for array indexing
                let row_idx = row_i16 as usize;
                let col_idx = col_i16 as usize;

                // Apply the command based on Day 6, Part 1 rules
                match &self.cmd {
                    Command::TurnOn => field_guard[row_idx][col_idx] = 1, // 1 for on
                    Command::TurnOff => field_guard[row_idx][col_idx] = 0, // 0 for off
                    Command::Toggle => {
                        // Flip 0 to 1, or any other value (assuming 1) to 0
                        field_guard[row_idx][col_idx] = match field_guard[row_idx][col_idx] {
                            0 => 1,
                            _ => 0, // Treat any non-zero as "on" for toggling
                        }
                    }
                }
            }
        }
    }

    /// Executes the instruction according to Day 6, Part 2 rules.
    ///
    /// Modifies the global `FIELD` grid, which now represents brightness levels.
    /// - `TurnOn`: Increases brightness by 1.
    /// - `TurnOff`: Decreases brightness by 1, but not below 0.
    /// - `Toggle`: Increases brightness by 2.
    ///
    /// # Panics
    /// Panics if the `FIELD` mutex cannot be locked.
    fn execute_right(&self) {
        println!(
            "executing instruction {:?} in range {:?} through {:?}",
            self.cmd, self.corners.0, self.corners.1
        );

        // Acquire a mutable lock on the global FIELD to modify it
        let mut field_guard = FIELD.lock().unwrap();

        // Iterate over the rectangular region defined by the instruction's corners
        for row_i16 in self.corners.0.y..=self.corners.1.y {
            for col_i16 in self.corners.0.x..=self.corners.1.x {
                // Cast i16 coordinates to usize for array indexing
                let row_idx = row_i16 as usize;
                let col_idx = col_i16 as usize;

                // Apply the command based on Day 6, Part 2 rules
                match &self.cmd {
                    Command::TurnOn => field_guard[row_idx][col_idx] += 1, // Increase brightness by 1
                    Command::TurnOff => {
                        // Decrease brightness by 1, but ensure it doesn't go below 0
                        if field_guard[row_idx][col_idx] > 0 {
                            field_guard[row_idx][col_idx] -= 1;
                        }
                    }
                    Command::Toggle => field_guard[row_idx][col_idx] += 2, // Increase brightness by 2
                }
            }
        }
    }
}

/// Decodes a slice of raw instruction strings into an iterator of `Instruction` structs.
///
/// This function converts `&str` inputs into `Instruction` structs, unwrapping any parsing errors.
/// In a production scenario, you might want to collect and report errors rather than unwrapping.
///
/// # Arguments
///
/// * `input` - A slice of string slices, each representing a raw instruction.
///
/// # Returns
///
/// An iterator that yields parsed `Instruction` structs.
fn decode_instructions(input: &[&str]) -> impl Iterator<Item = Instruction> {
    input
        .iter()
        .map(|&s| Instruction::new(s)) // Attempt to parse each string into an Instruction
        .map(|x| x.unwrap()) // Unwrap the Result. Panics on parsing errors.
}

/// Initialises the global `FIELD` grid by setting all cells to 0.
///
/// This function is called before processing instructions for each part to ensure a clean state.
/// # Panics
/// Panics if the `FIELD` mutex cannot be locked.
fn init_field() {
    let mut field_guard = FIELD.lock().unwrap(); // Acquire mutable lock
    for row in 0..ROWS {
        for col in 0..COLS {
            field_guard[row][col] = 0; // Set cell to 0
        }
    }
}

/// Solves Day 6, Part 1: Counts the number of lights that are "on" (value > 0) after
/// executing all instructions according to Part 1 rules.
///
/// # Arguments
///
/// * `input` - A slice of raw instruction strings.
///
/// # Returns
///
/// The total count of lights that are on.
/// # Panics
/// Panics if instruction parsing fails or if the `FIELD` mutex cannot be locked.
pub fn solve_part1(input: &[&str]) -> i32 {
    let instructions = decode_instructions(input); // Parse instructions
    let _ = init_field(); // Initialise the grid
    instructions.for_each(|instr| instr.execute_wrong()); // Execute instructions using Part 1 logic

    // Acquire the lock to read the final state of the grid
    let field_guard = FIELD.lock().unwrap();

    // Iterate through the grid, count cells with value > 0, and sum them up
    field_guard
        .iter()
        .map(|row| row.iter().filter(|&&x| x > 0).count()) // Count 'on' lights (value > 0) per row
        .sum::<usize>() // Sum up the counts from all rows
        .try_into() // Convert the usize sum to i32
        .unwrap() // Panics if the sum exceeds i32::MAX
}

/// Solves Day 6, Part 2: Calculates the total brightness of all lights after
/// executing all instructions according to Part 2 rules.
///
/// # Arguments
///
/// * `input` - A slice of raw instruction strings.
///
/// # Returns
///
/// The total brightness of all lights.
/// # Panics
/// Panics if instruction parsing fails or if the `FIELD` mutex cannot be locked.
pub fn solve_part2(input: &[&str]) -> i32 {
    let instructions = decode_instructions(input); // Parse instructions
    let _ = init_field(); // Initialise the grid
    instructions.for_each(|instr| instr.execute_right()); // Execute instructions using Part 2 logic

    // Acquire the lock to read the final state of the grid
    let field_guard = FIELD.lock().unwrap();

    // Iterate through the grid, sum up all brightness values
    field_guard
        .iter()
        .map(|row| row.iter().sum::<i32>()) // Sum brightness per row
        .sum() // Sum up the total brightness from all rows (inferred as i32)
}

/// Contains unit tests for Day 6 solutions, verifying the logic against
/// examples provided in the Advent of Code problem description.
#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

    /// Tests `solve_part1` with a custom input to verify the light counting logic.
    #[test]
    fn test_part1() {
        let input = [
            "turn on 0,0 through 999,999", // All 1,000,000 lights on
            "toggle 0,0 through 999,0", // First row (1000 lights) toggled from 1 to 0. Remaining 999*1000 = 999000 on.
            // So, 1_000_000 - 1_000 = 999_000 lights on.
            "turn off 499,499 through 500,500", // A 2x2 square (4 lights) turned off.
                                                // These 4 lights were previously ON (value 1).
                                                // If they were not in the first row, then 999_000 - 4 = 998_996.
                                                // The example assumes these are not in the first row's toggled off section.
        ];

        // The example's logic seems to imply the affected lights were originally ON (value 1).
        // 1,000,000 (all on) - 1,000 (first row toggled off) = 999,000
        // 999,000 - 4 (2x2 square turned off) = 998,996
        assert_eq!(998_996, solve_part1(&input));
    }

    /// Tests `solve_part2` with a custom input to verify the total brightness calculation.
    #[test]
    fn test_part2() {
        let input = [
            "turn on 0,0 through 0,0", // One light, brightness +1. Total: 1
            "toggle 0,0 through 999,999", // All 1,000,000 lights get brightness +2.
                                       // The light at 0,0 becomes 1+2=3.
                                       // Other 999,999 lights become 0+2=2.
                                       // Total brightness: 3 + (999_999 * 2) = 3 + 1_999_998 = 2_000_001
        ];

        assert_eq!(2_000_001, solve_part2(&input));
    }
}
