//! Advent of Code 2015 - Day 2: I Was Told There Would Be No Math
//!
//! This module contains the solution for Day 2 of Advent of Code 2015,
//! which involves calculating the amount of wrapping paper and ribbon needed
//! for a list of presents with given dimensions.

use aoc_lib::read_multiple_strings;

/// Main entry point of the Day 2 solution.
///
/// Reads the input file for Day 2, processes it to calculate
/// the total wrapping paper and ribbon needed, and prints the results for Part 1 and Part 2.
fn main() {
    let lines_result = read_multiple_strings(2);

    // Use `if let Ok(...)` for better error handling than `if .is_ok() .unwrap()`
    if let Ok(owned_lines_vec) = lines_result {
        // `owned_lines_vec` now owns the Vec<String> data.

        // Create a Vec<&str> where each &str borrows from the Strings inside `owned_lines_vec`.
        let input_line_slices: Vec<&str> = owned_lines_vec
            .iter()
            .map(|s| s.as_str()) // Convert `&String` to `&str`
            .collect(); // Collect these `&str` into a new `Vec<&str>`

        // Now, `input_line_slices` is a `Vec<&str>` that borrows from `owned_lines_vec`.
        // We pass a reference (slice) to it, so it's not moved.
        println!("Day 2 Part 1: {}", solve_part1(&input_line_slices));
        println!("Day 2 Part 2: {}", solve_part2(&input_line_slices));
    } else {
        eprintln!("Error reading input for Day 2: {:?}", lines_result.err());
    }
}

/// Parses a dimensions string (e.g., "2x3x4") into a tuple of three `i32` integers.
///
/// Returns `Some((width, height, length))` if parsing is successful,
/// otherwise returns `None` if the input format is invalid or parts cannot be parsed.
///
/// # Arguments
///
/// * `input` - A string slice representing the dimensions (e.g., "LxWxH").
fn string_to_tuple(input: &str) -> Option<(i32, i32, i32)> {
    let mut iterator = input.split('x');

    let w: i32 = iterator.next()?.parse().ok()?;
    let h: i32 = iterator.next()?.parse().ok()?;
    let l: i32 = iterator.next()?.parse().ok()?;

    Some((w, h, l))
}

/// Calculates the total amount of wrapping paper needed for a present.
///
/// The formula is `2*lw + 2*wh + 2*hl + smallest_side_area`.
///
/// # Arguments
///
/// * `dimensions` - A tuple `(width, length, height)` representing the present's dimensions.
fn calculate_paper((w, l, h): (i32, i32, i32)) -> i32 {
    let areas = [l * w, w * h, h * l];
    let min_area = *areas.iter().min().unwrap();
    let areas_sum: i32 = areas.iter().map(|&x| x).sum();
    2 * areas_sum + min_area
}

/// Calculates the total length of ribbon needed for a present.
///
/// The formula is `smallest_perimeter + volume`.
/// The smallest perimeter is the sum of the two smallest sides times 2.
///
/// # Arguments
///
/// * `dimensions` - A tuple `(width, length, height)` representing the present's dimensions.
fn calculate_ribbon((w, l, h): (i32, i32, i32)) -> i32 {
    let perimeters = [2 * (l + w), 2 * (w + h), 2 * (h + l)];
    let min_perimeter = *perimeters.iter().min().unwrap();
    let ribbon_length = w * l * h;
    min_perimeter + ribbon_length
}

/// Solves Day 2 Part 1: Calculates the total square feet of wrapping paper needed.
///
/// Iterates through each line of input, parses it into present dimensions,
/// calculates the paper needed for each, and sums the results.
///
/// # Arguments
///
/// * `input` - A slice of string slices, where each string represents the dimensions of a present (e.g., "2x3x4").
pub fn solve_part1(input: &[&str]) -> i32 {
    let input_tuples = input
        .iter()
        .map(|as_string: &&str| string_to_tuple(as_string).unwrap());
    let dimensions = input_tuples.map(|t| calculate_paper(t));
    dimensions.sum()
}

/// Solves Day 2 Part 2: Calculates the total feet of ribbon needed.
///
/// Iterates through each line of input, parses it into present dimensions,
/// calculates the ribbon needed for each, and sums the results.
///
/// # Arguments
///
/// * `input` - A slice of string slices, where each string represents the dimensions of a present (e.g., "2x3x4").
pub fn solve_part2(input: &[&str]) -> i32 {
    let input_tuples = input
        .iter()
        .map(|as_string: &&str| string_to_tuple(as_string).unwrap());
    let ribbon_lengths = input_tuples.map(|t| calculate_ribbon(t));
    ribbon_lengths.sum()
}

/// Contains unit tests for Day 2 solutions.
#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

    /// Tests `solve_part1` with example "2x3x4" from Advent of Code.
    #[test]
    fn test_solve_part1_example_a() {
        let example_input = "2x3x4";
        let converted_input: Vec<&str> = Vec::from([example_input]);
        assert_eq!(58, solve_part1(&converted_input));
    }

    /// Tests `solve_part1` with example "1x1x10" from Advent of Code.
    #[test]
    fn test_solve_part1_example_b() {
        let example_input = "1x1x10";
        let converted_input: Vec<&str> = Vec::from([example_input]);
        assert_eq!(43, solve_part1(&converted_input));
    }

    /// Tests `solve_part2` with example "2x3x4" from Advent of Code.
    #[test]
    fn test_solve_part2_example_a() {
        let example_input = "2x3x4";
        let converted_input: Vec<&str> = Vec::from([example_input]);
        assert_eq!(34, solve_part2(&converted_input));
    }

    /// Tests `solve_part2` with example "1x1x10" from Advent of Code.
    #[test]
    fn test_solve_part2_example_b() {
        let example_input = "1x1x10";
        let converted_input: Vec<&str> = Vec::from([example_input]);
        assert_eq!(14, solve_part2(&converted_input));
    }
}
