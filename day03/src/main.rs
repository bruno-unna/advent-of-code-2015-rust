//! Advent of Code 2015 - Day 3: Perfectly Spherical Houses in a Vacuum
//!
//! This module contains the solution for Day 3 of Advent of Code 2015,
//! simulating Santa's and Robo-Santa's movements to count houses that receive at least one present.

use std::collections::HashSet;

use aoc_lib::read_single_string;

/// Main entry point of the Day 3 solution.
///
/// Reads the input string representing Santa's movement,
/// then calculates and prints the number of houses that receive at least one present
/// for both Part 1 (Santa alone) and Part 2 (Santa and Robo-Santa).
fn main() {
    let line_result = read_single_string(3);

    if let Ok(owned_line) = line_result {
        println!("Day 3 Part 1: {}", solve_part1(&owned_line));
        println!("Day 3 Part 2: {}", solve_part2(&owned_line));
    } else {
        eprintln!("Error reading input for Day 3: {:?}", line_result.err());
    }
}

/// Simulates a path taken by a gift-delivering entity and records all unique houses visited.
///
/// The simulation starts at position (0,0) and records it as visited.
/// It then iterates through the provided movement instructions,
/// updating the current position and adding each new position to the `visited` set.
///
/// # Type Parameters
///
/// * `I`: An iterator that yields `char` items, representing the movement instructions.
///
/// # Arguments
///
/// * `input_chars` - An iterator of characters (`^`, `v`, `>`, `<`) representing the path taken.
/// * `visited` - A mutable reference to a `HashSet<(i32, i32)>` where unique visited
///   coordinates will be inserted.
fn walk<I>(input_chars: I, visited: &mut HashSet<(i32, i32)>)
where
    I: Iterator<Item = char>,
{
    let mut position = (0, 0);
    visited.insert(position);
    for c in input_chars {
        let diff = match c {
            '^' => (0, -1),
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => (0, 0),
        };
        position.0 += diff.0;
        position.1 += diff.1;
        visited.insert(position);
    }
}

/// Solves Day 3 Part 1: Calculates the number of houses that receive at least one present
/// when only Santa delivers.
///
/// Santa follows all instructions in the `input` string.
///
/// # Arguments
///
/// * `input` - A string slice containing the movement instructions for Santa.
pub fn solve_part1(input: &str) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    walk(input.chars(), &mut visited);
    visited.len().try_into().unwrap()
}

/// Solves Day 3 Part 2: Calculates the number of houses that receive at least one present
/// when Santa and Robo-Santa deliver.
///
/// Santa takes the first instruction, Robo-Santa takes the second, Santa the third, and so on.
/// Both start at the same initial location (0,0) and deliver to their starting house.
///
/// # Arguments
///
/// * `input` - A string slice containing the interleaved movement instructions for Santa and Robo-Santa.
pub fn solve_part2(input: &str) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let santa_input = input.chars().step_by(2);
    let robo_santa_input = input.chars().skip(1).step_by(2);

    walk(santa_input, &mut visited);
    walk(robo_santa_input, &mut visited);

    visited.len().try_into().unwrap()
}

/// Contains unit tests for Day 3 solutions.
#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

    /// Tests `solve_part1` with example ">" which should visit 2 houses.
    #[test]
    fn test_solve_part1_example_a() {
        let example_input = ">";
        assert_eq!(2, solve_part1(example_input));
    }

    /// Tests `solve_part1` with example "^>v<" which should visit 4 houses.
    #[test]
    fn test_solve_part1_example_b() {
        let example_input = "^>v<";
        assert_eq!(4, solve_part1(example_input));
    }

    /// Tests `solve_part1` with example "^v^v^v^v^v" which should visit 2 houses.
    #[test]
    fn test_solve_part1_example_c() {
        let example_input = "^v^v^v^v^v";
        assert_eq!(2, solve_part1(example_input));
    }

    /// Tests `solve_part2` with example "^v" which should visit 3 houses (Santa: ^, Robo-Santa: v).
    #[test]
    fn test_solve_part2_example_a() {
        let example_input = "^v";
        assert_eq!(3, solve_part2(example_input));
    }

    /// Tests `solve_part2` with example "^>v<" which should visit 3 houses.
    #[test]
    fn test_solve_part2_example_b() {
        let example_input = "^>v<";
        assert_eq!(3, solve_part2(example_input));
    }

    /// Tests `solve_part2` with example "^v^v^v^v^v" which should visit 11 houses.
    #[test]
    fn test_solve_part2_example_c() {
        let example_input = "^v^v^v^v^v";
        assert_eq!(11, solve_part2(example_input));
    }
}
