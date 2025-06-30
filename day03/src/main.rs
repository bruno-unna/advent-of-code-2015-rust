use std::collections::HashSet;

use aoc_lib::read_single_string;

fn main() {
    let line_result = read_single_string(3);

    if let Ok(owned_line) = line_result {
        println!("Day 3 Part 1: {}", solve_part1(&owned_line));
        println!("Day 3 Part 2: {}", solve_part2(&owned_line));
    } else {
        eprintln!("Error reading input for Day 3: {:?}", line_result.err());
    }
}

fn walk<I>(input: I, visited: &mut HashSet<(i32, i32)>)
where
    I: Iterator<Item = char>,
{
    let mut position = (0, 0);
    visited.insert(position);
    for c in input {
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

pub fn solve_part1(input: &str) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    walk(input.chars(), &mut visited);
    visited.len().try_into().unwrap()
}

pub fn solve_part2(input: &str) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let santa_input = input.chars().step_by(2);
    let robo_santa_input = input.chars().skip(1).step_by(2);

    walk(santa_input, &mut visited);
    walk(robo_santa_input, &mut visited);

    visited.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

    #[test]
    fn test_solve_part1_example_a() {
        let example_input = ">";
        assert_eq!(2, solve_part1(example_input));
    }

    #[test]
    fn test_solve_part1_example_b() {
        let example_input = "^>v<";
        assert_eq!(4, solve_part1(example_input));
    }

    #[test]
    fn test_solve_part1_example_c() {
        let example_input = "^v^v^v^v^v";
        assert_eq!(2, solve_part1(example_input));
    }

    #[test]
    fn test_solve_part2_example_a() {
        let example_input = "^v";
        assert_eq!(3, solve_part2(example_input));
    }

    #[test]
    fn test_solve_part2_example_b() {
        let example_input = "^>v<";
        assert_eq!(3, solve_part2(example_input));
    }

    #[test]
    fn test_solve_part2_example_c() {
        let example_input = "^v^v^v^v^v";
        assert_eq!(11, solve_part2(example_input));
    }
}
