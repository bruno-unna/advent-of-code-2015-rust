
fn main() {
    let input = "yzbqklnj";
    println!("Day 4 Part 1: {}", solve_part1(input));
    println!("Day 4 Part 2: {}", solve_part2(input));
}

pub fn solve_part1(_input: &str) -> i32 {
    0
}

pub fn solve_part2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

    #[test]
    fn test_solve_part1_example_a() {
        let example_input = "abcdef";
        assert_eq!(609043, solve_part1(example_input));
    }

    #[test]
    fn test_solve_part1_example_b() {
        let example_input = "pqrstuv";
        assert_eq!(1048970, solve_part1(example_input));
    }
}
