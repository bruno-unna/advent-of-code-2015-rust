use aoc_lib::read_multiple_strings;

use regex::Regex;
use std::sync::LazyLock;

fn main() {
    let lines_result = read_multiple_strings(7);

    // Use `if let Ok(...)` for better error handling than `if .is_ok() .unwrap()`
    if let Ok(owned_lines_vec) = lines_result {
        let input_line_slices: Vec<&str> = owned_lines_vec
            .iter()
            .map(|s| s.as_str()) // Convert `&String` to `&str`
            .collect(); // Collect these `&str` into a new `Vec<&str>`

        println!("Day 7 Part 1: {}", solve_part1(&input_line_slices));
    } else {
        eprintln!("Error reading input: {:?}", lines_result.err());
    }
}

#[derive(Debug)]
enum GateType {
    Assignment,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

#[derive(Debug, Clone)]
enum Argument {
    Wire(String),
    Number(u16),
}

#[derive(Debug)]
struct Gate {
    gate_type: GateType,
    arguments: (Argument, Option<Argument>),
    destination: String,
}

static ASSIGNMENT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s*->\s*([a-z]+)$").unwrap());

static AND_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s+AND\s+(\d+|[a-z]+)\s*->\s*([a-z]+)$").unwrap());

static OR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s+OR\s+(\d+|[a-z]+)\s*->\s*([a-z]+)$").unwrap());

static LSHIFT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s+LSHIFT\s+(\d+)\s*->\s*([a-z]+)$").unwrap());

static RSHIFT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+|[a-z]+)\s+RSHIFT\s+(\d+)\s*->\s*([a-z]+)$").unwrap());

static NOT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^NOT\s+(\d+|[a-z]+)\s*->\s*([a-z]+)$").unwrap());

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

fn expand_arg(gates: &Vec<Gate>, arg: Argument) -> Option<u16> {
    match arg {
        Argument::Number(number) => Some(number),
        Argument::Wire(symbol) => run_circuit_for(gates, &symbol),
    }
}

fn run_circuit_for(gates: &Vec<Gate>, dest: &str) -> Option<u16> {
    let gate = gates.iter().find(|&g| g.destination == dest)?;
    if let Gate {
        gate_type: GateType::Assignment,
        arguments: (Argument::Number(n), _),
        destination: _,
    } = gate
    {
        return Some(*n);
    }
    match gate {
        Gate {
            gate_type: GateType::Assignment,
            arguments: (x, None),
            destination: _dest,
        } => return expand_arg(gates, x.clone()),
        Gate {
            gate_type: GateType::And,
            arguments: (x, Some(y)),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone());
            let b: Option<u16> = expand_arg(gates, y.clone());
            return a.zip(b).map(|(x, y)| x & y);
        }
        Gate {
            gate_type: GateType::Or,
            arguments: (x, Some(y)),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone());
            let b: Option<u16> = expand_arg(gates, y.clone());
            return a.zip(b).map(|(x, y)| x | y);
        }
        Gate {
            gate_type: GateType::LShift,
            arguments: (x, Some(Argument::Number(p))),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone());
            return a.map(|x| x << *p);
        }
        Gate {
            gate_type: GateType::RShift,
            arguments: (x, Some(Argument::Number(p))),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone());
            return a.map(|x| x >> *p);
        }
        Gate {
            gate_type: GateType::Not,
            arguments: (x, None),
            destination: _dest,
        } => {
            let a: Option<u16> = expand_arg(gates, x.clone());
            return a.map(|x| !x);
        }
        _ => None,
    }
}

pub fn solve_part1(input: &[&str]) -> u16 {
    let wires = parse_all_connections(input);
    run_circuit_for(&wires, "a").unwrap()
}

fn parse_all_connections(input: &[&str]) -> Vec<Gate> {
    input.iter().filter_map(|&c| parse_gate_str(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

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

        assert_eq!(72, run_circuit_for(&connections, "d").unwrap());
        assert_eq!(507, run_circuit_for(&connections, "e").unwrap());
        assert_eq!(492, run_circuit_for(&connections, "f").unwrap());
        assert_eq!(114, run_circuit_for(&connections, "g").unwrap());
        assert_eq!(65412, run_circuit_for(&connections, "h").unwrap());
        assert_eq!(65079, run_circuit_for(&connections, "i").unwrap());
        assert_eq!(123, run_circuit_for(&connections, "x").unwrap());
        assert_eq!(456, run_circuit_for(&connections, "y").unwrap());
    }
}
