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

pub fn solve_part1(input: &str) -> i32 {
    0
}

pub fn solve_part2(input: &str) -> i32 {
    0
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
}
