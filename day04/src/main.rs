//! Advent of Code 2015 - Day 4: The Ideal Stocking Stuffer
//!
//! This module contains the solution for Day 4 of Advent of Code 2015,
//! which involves finding an integer that, when appended to a secret key,
//! produces an MD5 hash with a specific number of leading zeroes.

use md5::{Digest, Md5};

/// Main entry point of the Day 4 solution.
///
/// Sets the Advent of Code puzzle input and prints the results for
/// Part 1 (5 leading zeroes) and Part 2 (6 leading zeroes).
fn main() {
    let input = "yzbqklnj";
    println!("Day 4 Part 1: {}", solve_part1(input));
    println!("Day 4 Part 2: {}", solve_part2(input));
}

/// Calculates the MD5 hash of a given byte slice and returns it as a hexadecimal string.
///
/// This helper function wraps the `md5` crate's functionality.
///
/// # Arguments
///
/// * `challenge` - A byte slice (`&[u8]`) representing the data to be hashed.
///
/// # Returns
///
/// A `String` containing the 32-character hexadecimal representation of the MD5 hash.
fn md5_sum(challenge: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(challenge);
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Finds the lowest positive `i64` integer `n` such that
/// the MD5 hash of `"{input}{n}"` starts with the specified `prefix`.
///
/// This function efficiently builds the string to be hashed by reusing a
/// pre-allocated buffer to avoid repeated memory allocations within the loop.
///
/// # Arguments
///
/// * `input` - The secret key string (the fixed prefix for the hash input).
/// * `prefix` - The hexadecimal string of leading zeroes (e.g., "00000") to match.
///
/// # Returns
///
/// The `i64` integer `n` that satisfies the condition.
fn find_number(input: &str, prefix: &str) -> i64 {
    const MAX_I64_DIGITS: usize = 19; // For a positive i64
    let mut challenge_buffer = String::with_capacity(input.len() + MAX_I64_DIGITS);

    let mut n: i64 = 0;

    loop {
        challenge_buffer.clear();
        challenge_buffer.push_str(input);
        challenge_buffer.push_str(&n.to_string());

        let md5 = md5_sum(challenge_buffer.as_bytes());
        if md5.starts_with(prefix) {
            break;
        }
        n += 1;
    }
    n
}

/// Solves Day 4 Part 1: Finds the lowest positive integer `n` such that
/// the MD5 hash of `"{input}{n}"` starts with five zeroes ("00000").
///
/// # Arguments
///
/// * `input` - The secret key string from the puzzle.
///
/// # Returns
///
/// The integer `n` that produces the required hash.
pub fn solve_part1(input: &str) -> i64 {
    find_number(input, "00000")
}

/// Solves Day 4 Part 2: Finds the lowest positive integer `n` such that
/// the MD5 hash of `"{input}{n}"` starts with six zeroes ("000000").
///
/// # Arguments
///
/// * `input` - The secret key string from the puzzle.
///
/// # Returns
///
/// The integer `n` that produces the required hash.
pub fn solve_part2(input: &str) -> i64 {
    find_number(input, "000000")
}

/// Contains unit tests for Day 4 solutions.
#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

    /// Tests `solve_part1` with example "abcdef" which should result in 609043.
    #[test]
    fn test_solve_part1_example_a() {
        let example_input = "abcdef";
        assert_eq!(609043, solve_part1(example_input));
    }

    /// Tests `solve_part1` with example "pqrstuv" which should result in 1048970.
    #[test]
    fn test_solve_part1_example_b() {
        let example_input = "pqrstuv";
        assert_eq!(1048970, solve_part1(example_input));
    }
}
