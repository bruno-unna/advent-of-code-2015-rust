//! Advent of Code 2015 - Day 7: "Some Assembly Required"
//!
//! This module implements a circuit simulator to determine the signal value
//! on specific wires based on a set of logical gates. It uses memoization
//! to handle dependencies and avoid redundant computations in the circuit.

use aoc_lib::read_multiple_strings;

use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Main entry point of the Day 7 solution.
///
/// Reads the input lines representing circuit gates, parses them,
/// and then prints the signal value for wire 'a' for Part 1.
fn main() {
    let lines_result = read_multiple_strings(7);

    if let Ok(owned_lines_vec) = lines_result {
        let input_line_slices: Vec<&str> = owned_lines_vec.iter().map(|s| s.as_str()).collect();

        let part_1 = solve_part1(&input_line_slices);
        println!("Day 7 Part 1: {}", part_1);
        println!("Day 7 Part 2: {}", solve_part2(&input_line_slices, part_1))
    } else {
        eprintln!("Error reading input: {:?}", lines_result.err());
    }
}

/// Represents the type of logical gate in the circuit.
#[derive(Debug)]
enum GateType {
    Assignment,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

/// Represents an argument to a gate, which can be either a wire symbol (String)
/// or a direct numerical value (u16).
#[derive(Debug, Clone)]
enum Argument {
    Wire(String),
    Number(u16),
}

/// Represents a single gate in the circuit.
///
/// Contains the type of gate, its input arguments (one or two), and the
/// destination wire where its output signal is placed.
#[derive(Debug)]
struct Gate {
    gate_type: GateType,
    arguments: (Argument, Option<Argument>),
    destination: String,
}

/// Regular expression for parsing assignment gates (e.g., "123 -> x" or "lx -> a").
static ASSIGNMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s*->\s*([a-z]+)$").unwrap());

/// Regular expression for parsing AND gates (e.g., "x AND y -> d").
static AND_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s+AND\s+(\d+|[a-z]+)\s*->\s*([a-z]+)$").unwrap());

/// Regular expression for parsing OR gates (e.g., "x OR y -> e").
static OR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s+OR\s+(\d+|[a-z]+)\s*->\s*([a-z]+)$").unwrap());

/// Regular expression for parsing LSHIFT gates (e.g., "x LSHIFT 2 -> f").
static LSHIFT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s+LSHIFT\s+(\d+)\s*->\s*([a-z]+)$").unwrap());

/// Regular expression for parsing RSHIFT gates (e.g., "y RSHIFT 2 -> g").
static RSHIFT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s+RSHIFT\s+(\d+)\s*->\s*([a-z]+)$").unwrap());

/// Regular expression for parsing NOT gates (e.g., "NOT x -> h").
static NOT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^NOT\s+(\d+|[a-z]+)\s*->\s*([a-z]+)$").unwrap());

/// Parses a single gate instruction string into an `Option<Gate>`.
///
/// It attempts to match the string against various gate regex patterns.
/// If a match is found, it extracts arguments and destination, converts
/// numbers/wires to `Argument` enum variants, and constructs a `Gate` struct.
///
/// # Arguments
///
/// * `gate_str` - The string representing a single gate instruction.
///
/// # Returns
///
/// `Some(Gate)` if parsing is successful, `None` otherwise.
fn parse_gate_str(gate_str: &str) -> Option<Gate> {
    if let Some(captures) = ASSIGNMENT_RE.captures(gate_str) {
        let arg: Argument = captures[1].parse::<u16>().map_or_else(
            |_e| Argument::Wire(captures[1].to_string()),
            |n| Argument::Number(n),
        );
        return Some(Gate {
            gate_type: GateType::Assignment,
            arguments: (arg, None),
            destination: captures[2].to_string(),
        });
    }
    if let Some(captures) = AND_RE.captures(gate_str) {
        let left_arg: Argument = captures[1].parse::<u16>().map_or_else(
            |_e| Argument::Wire(captures[1].to_string()),
            |n| Argument::Number(n),
        );
        let right_arg: Option<Argument> = Some(captures[2].parse::<u16>().map_or_else(
            |_e| Argument::Wire(captures[2].to_string()),
            |n| Argument::Number(n),
        ));
        return Some(Gate {
            gate_type: GateType::And,
            arguments: (left_arg, right_arg),
            destination: captures[3].to_string(),
        });
    }
    if let Some(captures) = OR_RE.captures(gate_str) {
        let left_arg: Argument = captures[1].parse::<u16>().map_or_else(
            |_e| Argument::Wire(captures[1].to_string()),
            |n| Argument::Number(n),
        );
        let right_arg: Option<Argument> = Some(captures[2].parse::<u16>().map_or_else(
            |_e| Argument::Wire(captures[2].to_string()),
            |n| Argument::Number(n),
        ));
        return Some(Gate {
            gate_type: GateType::Or,
            arguments: (left_arg, right_arg),
            destination: captures[3].to_string(),
        });
    }
    if let Some(captures) = LSHIFT_RE.captures(gate_str) {
        return Some(Gate {
            gate_type: GateType::LShift,
            arguments: (
                Argument::Wire(captures[1].to_string()),
                Some(Argument::Number(captures[2].parse::<u16>().unwrap())),
            ),
            destination: captures[3].to_string(),
        });
    }
    if let Some(captures) = RSHIFT_RE.captures(gate_str) {
        return Some(Gate {
            gate_type: GateType::RShift,
            arguments: (
                Argument::Wire(captures[1].to_string()),
                Some(Argument::Number(captures[2].parse::<u16>().unwrap())),
            ),
            destination: captures[3].to_string(),
        });
    }
    if let Some(captures) = NOT_RE.captures(gate_str) {
        let arg: Argument = captures[1].parse::<u16>().map_or_else(
            |_e| Argument::Wire(captures[1].to_string()),
            |n| Argument::Number(n),
        );
        return Some(Gate {
            gate_type: GateType::Not,
            arguments: (arg, None),
            destination: captures[2].to_string(),
        });
    }
    return None;
}

/// Expands an `Argument` to its `u16` signal value.
///
/// If the argument is a `Number`, it returns the number directly.
/// If the argument is a `Wire`, it recursively calls `run_circuit_for`
/// to determine the signal value of that wire.
///
/// # Arguments
///
/// * `gates` - A reference to the vector of all gates in the circuit.
/// * `arg` - The `Argument` to expand.
/// * `memo` - A mutable reference to the memoization `HashMap` to store/retrieve wire values.
///
/// # Returns
///
/// `Some(u16)` if the argument can be resolved to a signal value, `None` otherwise.
fn expand_arg(gates: &Vec<Gate>, arg: Argument, memo: &mut HashMap<String, u16>) -> Option<u16> {
    match arg {
        Argument::Number(number) => Some(number),
        Argument::Wire(symbol) => run_circuit_for(gates, &symbol, memo),
    }
}

/// Computes the signal value for a given destination wire.
///
/// This function uses memoization to store and retrieve already computed wire values,
/// preventing infinite recursion and redundant calculations.
/// It finds the gate that outputs to `dest` and recursively computes its input arguments.
///
/// # Arguments
///
/// * `gates` - A reference to the vector of all gates in the circuit.
/// * `dest` - The name of the destination wire whose signal value is to be computed.
/// * `memo` - A mutable reference to the memoization `HashMap` to store/retrieve wire values.
///
/// # Returns
///
/// `Some(u16)` if the signal value for `dest` can be computed, `None` if it cannot
/// (e.g., due to an unresolvable dependency).
fn run_circuit_for(gates: &Vec<Gate>, dest: &str, memo: &mut HashMap<String, u16>) -> Option<u16> {
    if let Some(&value) = memo.get(dest) {
        return Some(value);
    }

    let gate = gates.iter().find(|&g| g.destination == dest)?;

    if let Gate {
        gate_type: GateType::Assignment,
        arguments: (Argument::Number(n), _),
        destination: _,
    } = gate
    {
        let result = Some(*n);
        memo.insert(dest.to_string(), result.unwrap());
        return result;
    }

    let result = match gate {
        Gate {
            gate_type: GateType::Assignment,
            arguments: (x, None),
            destination: _dest,
        } => expand_arg(gates, x.clone(), memo),

        Gate {
            gate_type: GateType::And,
            arguments: (x, Some(y)),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone(), memo);
            let b: Option<u16> = expand_arg(gates, y.clone(), memo);
            a.zip(b).map(|(x, y)| x & y)
        }
        Gate {
            gate_type: GateType::Or,
            arguments: (x, Some(y)),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone(), memo);
            let b: Option<u16> = expand_arg(gates, y.clone(), memo);
            a.zip(b).map(|(x, y)| x | y)
        }
        Gate {
            gate_type: GateType::LShift,
            arguments: (x, Some(Argument::Number(p))),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone(), memo);
            a.map(|x| x << *p)
        }
        Gate {
            gate_type: GateType::RShift,
            arguments: (x, Some(Argument::Number(p))),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone(), memo);
            a.map(|x| x >> *p)
        }
        Gate {
            gate_type: GateType::Not,
            arguments: (x, None),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone(), memo);
            a.map(|x| !x)
        }
        _ => None,
    };

    if let Some(val) = result {
        memo.insert(dest.to_string(), val);
    }
    result
}

/// Solves Day 7, Part 1: Computes the final signal value on wire 'a'.
///
/// Parses all circuit gate instructions, then uses the `run_circuit_for`
/// function with memoization to determine the value of wire 'a'.
///
/// # Arguments
///
/// * `input` - A slice of string slices, each representing a gate instruction.
///
/// # Returns
///
/// The `u16` signal value on wire 'a'. Panics if 'a' cannot be resolved.
pub fn solve_part1(input: &[&str]) -> u16 {
    let gates = parse_all_connections(input);
    let mut memo: HashMap<String, u16> = HashMap::new();
    run_circuit_for(&gates, "a", &mut memo).unwrap()
}

/// Solves Day 7, Part 2: Computes the final signal value on wire 'a' after overriding wire 'b'.
///
/// This function re-initializes the circuit by parsing all connections. It then finds
/// the gate that outputs to wire 'b', removes it, modifies its input to be the
/// `part_1` result (the signal value from wire 'a' in Part 1), and re-inserts it.
/// Finally, it uses `run_circuit_for` with a fresh memoization cache to determine
/// the new signal value of wire 'a' with the modified circuit.
///
/// # Arguments
///
/// * `input` - A slice of string slices, each representing a circuit gate instruction.
/// * `part_1` - The `u16` signal value computed for wire 'a' in Part 1.
///
/// # Returns
///
/// The `u16` signal value on wire 'a' in the modified circuit.
///
/// # Panics
///
/// Panics if the gate for wire 'b' is not found, or if wire 'a' cannot be resolved.
pub fn solve_part2(input: &[&str], part_1: u16) -> u16 {
    let mut gates = parse_all_connections(input);
    let index_of_b = gates.iter().position(|g| g.destination == "b").unwrap();
    let mut b = gates.swap_remove(index_of_b);
    b.arguments.0 = Argument::Number(part_1);
    gates.push(b);

    let mut memo: HashMap<String, u16> = HashMap::new();
    run_circuit_for(&gates, "a", &mut memo).unwrap()
}

/// Parses all connection strings into a vector of `Gate` structs.
///
/// Filters out any strings that cannot be successfully parsed into a `Gate`.
///
/// # Arguments
///
/// * `input` - A slice of string slices, each representing a gate instruction.
///
/// # Returns
///
/// A `Vec<Gate>` containing all successfully parsed gates.
fn parse_all_connections(input: &[&str]) -> Vec<Gate> {
    input.iter().filter_map(|&c| parse_gate_str(c)).collect()
}

/// Contains unit tests for Day 7 solution, verifying the circuit logic
/// against examples provided in the Advent of Code problem description.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the circuit simulation with the example input from Part 1,
    /// verifying the signal values for various wires.
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

        let connections = parse_all_connections(&input);

        let mut memo_d = HashMap::new();
        assert_eq!(72, run_circuit_for(&connections, "d", &mut memo_d).unwrap());

        let mut memo_e = HashMap::new();
        assert_eq!(
            507,
            run_circuit_for(&connections, "e", &mut memo_e).unwrap()
        );

        let mut memo_f = HashMap::new();
        assert_eq!(
            492,
            run_circuit_for(&connections, "f", &mut memo_f).unwrap()
        );

        let mut memo_g = HashMap::new();
        assert_eq!(
            114,
            run_circuit_for(&connections, "g", &mut memo_g).unwrap()
        );

        let mut memo_h = HashMap::new();
        assert_eq!(
            65412,
            run_circuit_for(&connections, "h", &mut memo_h).unwrap()
        );

        let mut memo_i = HashMap::new();
        assert_eq!(
            65079,
            run_circuit_for(&connections, "i", &mut memo_i).unwrap()
        );

        let mut memo_x = HashMap::new();
        assert_eq!(
            123,
            run_circuit_for(&connections, "x", &mut memo_x).unwrap()
        );

        let mut memo_y = HashMap::new();
        assert_eq!(
            456,
            run_circuit_for(&connections, "y", &mut memo_y).unwrap()
        );
    }
}
