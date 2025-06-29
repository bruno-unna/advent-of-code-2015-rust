// day01/src/main.rs (This is a text file)
use aoc_lib::read_input;

fn main() {
    let input_text = read_input(1); // Read input from the text file for Day 1
    println!("Day 1 Part 1: {}", solve_part1(&input_text));
    println!("Day 1 Part 2: {}", solve_part2(&input_text));
}

pub fn solve_part1(input: &str) -> i32 { // Using i32 as example return type, adjust as needed
    // Your solution logic for Part 1 goes here
    let left_pars:i32 = input.chars().filter(|&c| c=='(').count().try_into().unwrap();
    let right_pars:i32 = input.chars().filter(|&c| c==')').count().try_into().unwrap();
    left_pars - right_pars
}

pub fn solve_part2(input: &str) -> i32 { // Using i32 as example return type, adjust as needed
    // Your solution logic for Part 2 goes here
    input.lines().count() as i32 // Placeholder implementation
}


// --- Unit Tests Section ---
// The `#[cfg(test)]` attribute means this module is only compiled when running tests (`cargo test`).
#[cfg(test)]
mod tests {
    // The `use super::*` brings all items from the outer scope (like solve_part1, solve_part2)
    // into this `tests` module, so you can easily call them.
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

    // You can also test Part 2, assuming its examples are available
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

    // You can also write tests that don't directly call solve_part1/2
    #[test]
    fn it_works() {
        // Basic assertion
        assert!(true);
        // Another assertion: panic if values are not equal
        assert_eq!(2 + 2, 4);
        // Another assertion: panic if values ARE equal
        assert_ne!(2 + 2, 5);
    }
}