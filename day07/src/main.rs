use aoc_lib::read_multiple_strings;

use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

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
    Number(u16),
}

#[derive(Debug)]
struct Connection {
    connection_type: Type,
    arguments: (Argument, Option<Argument>),
    destination: String,
}

static ASSIGNMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+)\s*->\s*([a-z]+)$").unwrap());

static AND_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+)\s+AND\s+([a-z]+)\s*->\s*([a-z]+)$").unwrap());

static OR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+)\s+OR\s+([a-z]+)\s*->\s*([a-z]+)$").unwrap());

static LSHIFT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+)\s+LSHIFT\s+(\d+)\s*->\s*([a-z]+)$").unwrap());

static RSHIFT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+)\s+RSHIFT\s+(\d+)\s*->\s*([a-z]+)$").unwrap());

static NOT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^NOT\s+([a-z]+)\s*->\s*([a-z]+)$").unwrap());

fn parse_connection(connection_str: &str) -> Option<Connection> {
    if let Some(captures) = ASSIGNMENT_RE.captures(connection_str) {
        return Some(Connection {
            connection_type: Type::Assignment,
            arguments: (Argument::Number(captures[1].parse::<u16>().unwrap()), None),
            destination: captures[2].to_string(),
        });
    }
    if let Some(captures) = AND_RE.captures(connection_str) {
        return Some(Connection {
            connection_type: Type::And,
            arguments: (
                Argument::Wire(captures[1].to_string()),
                Some(Argument::Wire(captures[2].to_string())),
            ),
            destination: captures[3].to_string(),
        });
    }
    if let Some(captures) = OR_RE.captures(connection_str) {
        return Some(Connection {
            connection_type: Type::Or,
            arguments: (
                Argument::Wire(captures[1].to_string()),
                Some(Argument::Wire(captures[2].to_string())),
            ),
            destination: captures[3].to_string(),
        });
    }
    if let Some(captures) = LSHIFT_RE.captures(connection_str) {
        return Some(Connection {
            connection_type: Type::LShift,
            arguments: (
                Argument::Wire(captures[1].to_string()),
                Some(Argument::Number(captures[2].parse::<u16>().unwrap())),
            ),
            destination: captures[3].to_string(),
        });
    }
    if let Some(captures) = RSHIFT_RE.captures(connection_str) {
        return Some(Connection {
            connection_type: Type::RShift,
            arguments: (
                Argument::Wire(captures[1].to_string()),
                Some(Argument::Number(captures[2].parse::<u16>().unwrap())),
            ),
            destination: captures[3].to_string(),
        });
    }
    if let Some(captures) = NOT_RE.captures(connection_str) {
        return Some(Connection {
            connection_type: Type::Not,
            arguments: (Argument::Wire(captures[1].to_string()), None),
            destination: captures[2].to_string(),
        });
    }
    return None;
}

fn connect(connection: &Connection, result: &mut HashMap<String, u16>) {
    match connection {
        Connection {
            connection_type: Type::Assignment,
            arguments: (Argument::Number(n), None),
            destination: dest,
        } => result.insert((*dest).clone(), *n),
        Connection {
            connection_type: Type::And,
            arguments: (Argument::Wire(x), Some(Argument::Wire(y))),
            destination: dest,
        } => result.insert(
            (*dest).clone(),
            result.get(x).unwrap() & result.get(y).unwrap(),
        ),
        Connection {
            connection_type: Type::Or,
            arguments: (Argument::Wire(x), Some(Argument::Wire(y))),
            destination: dest,
        } => result.insert(
            (*dest).clone(),
            result.get(x).unwrap() | result.get(y).unwrap(),
        ),
        Connection {
            connection_type: Type::LShift,
            arguments: (Argument::Wire(x), Some(Argument::Number(p))),
            destination: dest,
        } => result.insert((*dest).clone(), result.get(x).unwrap() << *p),
        Connection {
            connection_type: Type::RShift,
            arguments: (Argument::Wire(x), Some(Argument::Number(p))),
            destination: dest,
        } => result.insert((*dest).clone(), result.get(x).unwrap() >> *p),
        Connection {
            connection_type: Type::Not,
            arguments: (Argument::Wire(x), None),
            destination: dest,
        } => result.insert((*dest).clone(), !result.get(x).unwrap()),
        _ => todo!(),
    };
}

fn run_circuit(connections: &[Connection]) -> HashMap<String, u16> {
    let mut result = HashMap::new();
    connections
        .iter()
        .for_each(|connection| connect(&connection, &mut result));
    result
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
        let parsed_input = input.map(|gate_str| parse_connection(gate_str).unwrap());
        let mut result: HashMap<String, u16> = HashMap::new();
        result.insert("d".to_string(), 72);
        result.insert("e".to_string(), 507);
        result.insert("f".to_string(), 492);
        result.insert("g".to_string(), 114);
        result.insert("h".to_string(), 65412);
        result.insert("i".to_string(), 65079);
        result.insert("x".to_string(), 123);
        result.insert("y".to_string(), 456);

        assert_eq!(result, run_circuit(&parsed_input));
    }
}
