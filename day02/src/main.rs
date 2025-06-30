use aoc_lib::read_input;

fn main() {
    let input_text = read_input(2); // Read input from the text file for Day 1
    println!("Day 2 Part 1: {}", solve_part1(&input_text));
    println!("Day 2 Part 2: {}", solve_part2(&input_text));
}

pub fn solve_part1(input: &str) -> i32 {
    0
}

pub fn solve_part2(input: &str) -> i32 {
    0
}

// --- Unit Tests Section ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_example_a() {
        let example_input = "2x3x4";
        assert_eq!(52, solve_part1(example_input));
    }

    #[test]
    fn test_solve_part1_example_b() {
        let example_input = "1x1x10";
        assert_eq!(43, solve_part1(example_input));
    }
}
