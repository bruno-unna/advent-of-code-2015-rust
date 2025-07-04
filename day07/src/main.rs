use aoc_lib::read_multiple_strings;

use regex::Regex;
use std::{collections::HashMap, sync::{LazyLock, Mutex}}; // LazyLock for lazy static initialisation, Mutex for thread-safe mutable static // Used for parsing instruction strings

fn main() {
    let lines_result = read_multiple_strings(7);

    // Use `if let Ok(...)` for better error handling than `if .is_ok() .unwrap()`
    if let Ok(owned_lines_vec) = lines_result {
        let input_line_slices: Vec<&str> = owned_lines_vec
            .iter()
            .map(|s| s.as_str()) // Convert `&String` to `&str`
            .collect(); // Collect these `&str` into a new `Vec<&str>`

        println!("Day 7 Part 1: {}", solve_part1(&input_line_slices));
        println!("Day 7 Part 2: {}", solve_part2(&input_line_slices));
    } else {
        eprintln!("Error reading input: {:?}", lines_result.err());
    }
}

#[derive(Debug)]
enum Type {
    Assignment,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

#[derive(Debug)]
enum Argument {
    Wire(String),
    Number(i8),
}

#[derive(Debug)]
struct Connection<'a> {
    connection_type: Type,
    argument: Argument,
    destination: &'a str,
}

static CMD_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap()
});

fn parse_connection(connection_str: &str) -> Connection {
    Connection::new()
}

fn run_circuit<'a>(connections: &'a [Connection<'a>]) -> HashMap<&'a str, u16> {
    HashMap::new()
}

pub fn solve_part1(input: &[&str]) -> u16 {
    0
}

pub fn solve_part2(input: &[&str]) -> u16 {
    0
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
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];
        let parsed_input = input.map(|gate_str| parse_connection(gate_str));
        let mut result = HashMap::new();
        result.insert("d", 72);
        result.insert("e", 507);
        result.insert("f", 492);
        result.insert("g", 114);
        result.insert("h", 65412);
        result.insert("i", 65079);
        result.insert("x", 123);
        result.insert("y", 456);

        assert_eq!(result, run_circuit(&parsed_input));
    }
}
