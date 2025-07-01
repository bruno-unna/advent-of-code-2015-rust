use md5::{Digest, Md5};

fn main() {
    let input = "yzbqklnj";
    println!("Day 4 Part 1: {}", solve_part1(input));
    println!("Day 4 Part 2: {}", solve_part2(input));
}

fn md5_sum(challenge: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(challenge);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn find_number(input: &str, prefix: &str) -> i64 {
    const MAX_I64_DIGITS: usize = 19; // For a positive i64
    let mut challenge_buffer = String::with_capacity(input.len() + MAX_I64_DIGITS);

    let mut n: i64 = 0;

    loop {
        challenge_buffer.clear();
        challenge_buffer.push_str(input);
        challenge_buffer.push_str(&n.to_string());

        let md5 = md5_sum(challenge_buffer.as_bytes());
        if md5.starts_with(prefix) {
            break;
        }
        n += 1;
    }
    n
}

pub fn solve_part1(input: &str) -> i64 {
    find_number(input, "00000")
}

pub fn solve_part2(input: &str) -> i64 {
    find_number(input, "000000")
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
