//! Advent of Code 2015 - Day 5: "Doesn't He Have Intern-Elves For This?"
//!
//! This module contains the solution for Day 5 of Advent of Code 2015,
//! which involves determining if strings are "nice" or "very nice" based
//! on specific character pattern rules. It leverages `fancy_regex` for
//! advanced regular expression matching, including backreferences.

use aoc_lib::read_multiple_strings;
use fancy_regex::Regex; // Alias for fancy_regex::Regex to avoid ambiguity with std::sync::LazyLock
use std::sync::LazyLock; // Used for lazy static initialisation of regexes

/// Main entry point of the Day 5 solution.
///
/// Reads the input lines, converts them to string slices, and then
/// prints the results for Part 1 (counting "nice" strings) and
/// Part 2 (counting "very nice" strings).
fn main() {
    let lines_result = read_multiple_strings(5);

    // Use `if let Ok(...)` for better error handling than `if .is_ok() .unwrap()`
    if let Ok(owned_lines_vec) = lines_result {
        let input_line_slices: Vec<&str> = owned_lines_vec
            .iter()
            .map(|s| s.as_str()) // Convert `&String` to `&str`
            .collect(); // Collect these `&str` into a new `Vec<&str>`

        println!("Day 5 Part 1: {}", solve_part1(&input_line_slices));
        println!("Day 5 Part 2: {}", solve_part2(&input_line_slices));
    } else {
        eprintln!("Error reading input: {:?}", lines_result.err());
    }
}

/// A `LazyLock` for a regular expression that matches strings containing
/// at least three vowels (a, e, i, o, u).
static THREE_VOWELS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap());

/// A `LazyLock` for a regular expression that matches strings containing
/// any of the forbidden substrings: "ab", "cd", "pq", or "xy".
static FORBIDDEN_COMBINATIONS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"ab|cd|pq|xy").unwrap());

/// A `LazyLock` for a regular expression that matches strings containing
/// at least one letter that appears twice in a row (e.g., "xx", "aa").
/// This uses a backreference `\1` and requires `fancy_regex`.
static TWICE_IN_A_ROW_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.)\1").unwrap());

/// Determines if a given `haystack` string is "nice" according to Day 5, Part 1 rules.
///
/// A string is nice if it meets all three conditions:
/// 1. It contains at least three vowels (aeiou only).
/// 2. It contains at least one letter that appears twice in a row.
/// 3. It does not contain the substrings "ab", "cd", "pq", or "xy".
///
/// # Arguments
///
/// * `haystack` - The string to evaluate.
///
/// # Returns
///
/// `true` if the string is nice, `false` otherwise.
fn is_nice(haystack: &str) -> bool {
    // Check for forbidden combinations first, as it's a quick fail.
    if FORBIDDEN_COMBINATIONS_RE.is_match(haystack).unwrap() {
        return false;
    }
    // Check for at least three vowels.
    if THREE_VOWELS_RE.is_match(haystack).unwrap() == false {
        return false;
    }
    // Check for a letter appearing twice in a row.
    if TWICE_IN_A_ROW_RE.is_match(haystack).unwrap() == false {
        return false;
    }
    true
}

/// A `LazyLock` for a regular expression that matches strings containing
/// a pair of any two letters that appears twice without overlapping
/// (e.g., "xyxy" contains "xy" twice, "aabcdefgaa" contains "aa" twice).
static DOUBLE_PAIRS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(..).*\1").unwrap());

/// A `LazyLock` for a regular expression that matches strings containing
/// at least one letter which repeats with exactly one letter between them
/// (e.g., "xyx", "abcdefe").
static SEPARATED_PAIR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.).\1").unwrap());

/// Determines if a given `haystack` string is "very nice" according to Day 5, Part 2 rules.
///
/// A string is very nice if it meets both conditions:
/// 1. It contains a pair of any two letters that appears at least twice without overlapping.
/// 2. It contains at least one letter which repeats with exactly one letter between them.
///
/// # Arguments
///
/// * `haystack` - The string to evaluate.
///
/// # Returns
///
/// `true` if the string is very nice, `false` otherwise.
fn is_very_nice(haystack: &str) -> bool {
    // Check for the double pair condition.
    if DOUBLE_PAIRS_RE.is_match(haystack).unwrap() == false {
        return false;
    }
    // Check for the separated pair condition.
    if SEPARATED_PAIR.is_match(haystack).unwrap() == false {
        return false;
    }
    true
}

/// Solves Day 5 Part 1: Counts the number of "nice" strings in the input.
///
/// # Arguments
///
/// * `input` - A slice of string slices, where each slice is a line from the puzzle input.
///
/// # Returns
///
/// The count of strings that meet the "nice" criteria.
pub fn solve_part1(input: &[&str]) -> i32 {
    input
        .iter()
        .filter(|&x| is_nice(x))
        .count()
        .try_into()
        .unwrap() // Convert usize to i32, panics on overflow (not expected for AoC counts)
}

/// Solves Day 5 Part 2: Counts the number of "very nice" strings in the input.
///
/// # Arguments
///
/// * `input` - A slice of string slices, where each slice is a line from the puzzle input.
///
/// # Returns
///
/// The count of strings that meet the "very nice" criteria.
pub fn solve_part2(input: &[&str]) -> i32 {
    input
        .iter()
        .filter(|&x| is_very_nice(x))
        .count()
        .try_into()
        .unwrap() // Convert usize to i32, panics on overflow (not expected for AoC counts)
}

/// Contains unit tests for Day 5 solutions, verifying the `is_nice` and `is_very_nice` functions
/// against examples provided in the Advent of Code problem description.
#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

    /// Tests `is_nice` with "ugknbfddgicrmopn", which should be nice.
    #[test]
    fn test_is_nice_example_a() {
        assert_eq!(true, is_nice("ugknbfddgicrmopn"));
    }

    /// Tests `is_nice` with "aaa", which should be nice.
    #[test]
    fn test_is_nice_example_b() {
        assert_eq!(true, is_nice("aaa"));
    }

    /// Tests `is_nice` with "jchzalrnumimnmhp", which should not be nice
    /// (due to lack of twice-in-a-row letter).
    #[test]
    fn test_is_nice_example_c() {
        assert_eq!(false, is_nice("jchzalrnumimnmhp"));
    }

    /// Tests `is_nice` with "haegwjzuvuyypxyu", which should not be nice
    /// (due to forbidden "xy" combination).
    #[test]
    fn test_is_nice_example_d() {
        assert_eq!(false, is_nice("haegwjzuvuyypxyu"));
    }

    /// Tests `is_nice` with "dvszwmarrgswjxmb", which should not be nice
    /// (due to lack of three vowels).
    #[test]
    fn test_is_nice_example_e() {
        assert_eq!(false, is_nice("dvszwmarrgswjxmb"));
    }

    /// Tests `is_very_nice` with "qjhvhtzxzqqjkmpb", which should be very nice.
    #[test]
    fn test_is_very_nice_example_a() {
        assert_eq!(true, is_very_nice("qjhvhtzxzqqjkmpb"));
    }

    /// Tests `is_very_nice` with "xxyxx", which should be very nice.
    #[test]
    fn test_is_very_nice_example_b() {
        assert_eq!(true, is_very_nice("xxyxx"));
    }

    /// Tests `is_very_nice` with "uurcxstgmygtbstg", which should not be very nice
    /// (lacks separated pair 'uu').
    #[test]
    fn test_is_very_nice_example_c() {
        assert_eq!(false, is_very_nice("uurcxstgmygtbstg"));
    }

    /// Tests `is_very_nice` with "ieodomkazucvgmuy", which should not be very nice
    /// (lacks double pair and separated pair).
    #[test]
    fn test_is_very_nice_example_d() {
        assert_eq!(false, is_very_nice("ieodomkazucvgmuy"));
    }
}
