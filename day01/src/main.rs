//! Day 1: Not Quite Lisp
//!
//! This module contains the solutions for Day 1 of Advent of Code 2015.
//! It calculates the final floor Santa ends up on and the position of the first
//! character that causes him to enter the basement.

use aoc_lib::read_input;

/// The main function for Day 1.
///
/// Reads the input file for Day 1 and prints the solutions for Part 1 and Part 2.
fn main() {
    let input_text = read_input(1); // Read input from the text file for Day 1
    println!("Day 1 Part 1: {}", solve_part1(&input_text));
    println!("Day 1 Part 2: {}", solve_part2(&input_text));
}

/// Calculates the final floor Santa ends up on.
///
/// Santa starts on the ground floor (floor 0). An opening parenthesis `(` means
/// he goes up one floor, and a closing parenthesis `)` means he goes down one floor.
///
/// # Arguments
///
/// * `input` - A string slice representing the sequence of instructions
///             (e.g., `(())`, `()()`, `(((`).
///
/// # Returns
///
/// The final floor number Santa reaches as an `i32`.
///
/// # Panics
///
/// This function will panic if the number of characters in the input string
/// exceeds the maximum value that can be represented by `i32`, due to the
/// `count().try_into().unwrap()` calls. Given the AoC constraints, this is unlikely.
///
/// # Examples
///
/// ```
/// // Assuming 'day01' is the crate name, and solve_part1 is public
/// // In a real project, you might need `use day01::solve_part1;` at the top of your test file.
/// assert_eq!(day01::solve_part1("(())"), 0);
/// assert_eq!(day01::solve_part1("()()"), 0);
/// assert_eq!(day01::solve_part1("((("), 3);
/// ```
pub fn solve_part1(input: &str) -> i32 {
    let left_pars: i32 = input
        .chars()
        .filter(|&c| c == '(')
        .count()
        .try_into()
        .unwrap();
    let right_pars: i32 = input
        .chars()
        .filter(|&c| c == ')')
        .count()
        .try_into()
        .unwrap();
    left_pars - right_pars
}

/// Finds the position of the first character that causes Santa to enter the basement (floor -1).
///
/// Santa starts on the ground floor (floor 0). An opening parenthesis `(` means
/// he goes up one floor, and a closing parenthesis `)` means he goes down one floor.
/// Positions are 1-indexed.
///
/// # Arguments
///
/// * `input` - A string slice representing the sequence of instructions.
///
/// # Returns
///
/// The 1-indexed position of the character that first causes Santa to enter
/// floor -1. If Santa never enters the basement, it returns 0.
///
/// # Examples
///
/// ```
/// // Assuming 'day01' is the crate name, and solve_part2 is public
/// // In a real project, you might need `use day01::solve_part2;` at the top of your test file.
/// assert_eq!(day01::solve_part2(")"), 1);
/// assert_eq!(day01::solve_part2("()())"), 5);
/// assert_eq!(day01::solve_part2("((("), 0); // Santa never reaches the basement
/// ```
pub fn solve_part2(input: &str) -> i32 {
    let mut pointer = 0;

    let r = input
        .chars()
        .scan(1, |floor, ch| {
            *floor += match ch {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            pointer += 1;
            return if *floor < 0 { None } else { Some(pointer) };
        })
        .last();

    r.unwrap_or(0)
}

// --- Unit Tests Section ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_example_a() {
        let example_input = "(())";
        assert_eq!(solve_part1(example_input), 0);
    }

    #[test]
    fn test_solve_part1_example_b() {
        let example_input = "()()";
        assert_eq!(solve_part1(example_input), 0);
    }

    #[test]
    fn test_solve_part1_example_c() {
        let example_input = "(((";
        assert_eq!(solve_part1(example_input), 3);
    }

    #[test]
    fn test_solve_part1_example_d() {
        let example_input = "(()(()(";
        assert_eq!(solve_part1(example_input), 3);
    }

    #[test]
    fn test_solve_part2_example_a() {
        let example_input = ")";
        assert_eq!(solve_part2(example_input), 1);
    }

    #[test]
    fn test_solve_part2_example_b() {
        let example_input = "()())";
        assert_eq!(solve_part2(example_input), 5);
    }

    #[test]
    fn test_solve_part2_example_c() {
        let example_input = "(((";
        assert_eq!(solve_part2(example_input), 0);
    }
}
