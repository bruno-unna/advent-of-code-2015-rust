// day01/src/main.rs (This is a text file)
use aoc_lib::read_input;

fn main() {
    let input_text = read_input(1); // Read input from the text file for Day 1
    println!("Day 1 Part 1: {}", solve_part1(&input_text));
    println!("Day 1 Part 2: {}", solve_part2(&input_text));
}

pub fn solve_part1(input: &str) -> i32 {
    // Using i32 as example return type, adjust as needed
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
// The `#[cfg(test)]` attribute means this module is only compiled when running tests (`cargo test`).
#[cfg(test)]
mod tests {
    // The `use super::*` brings all items from the outer scope (like solve_part1, solve_part2) into this `tests` module
    use super::*;

    // `#[test]` marks a function as a test function.
    #[test]
    fn test_solve_part1_example_a() {
        let example_input = "(())";
        assert_eq!(solve_part1(example_input), 0); // Assert that the result equals the expected value
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
        let example_input = ")"; // Example from Day 1, Part 2
        assert_eq!(solve_part2(example_input), 1);
    }

    #[test]
    fn test_solve_part2_example_b() {
        let example_input = "()())"; // Example from Day 1, Part 2
        assert_eq!(solve_part2(example_input), 5);
    }
}
