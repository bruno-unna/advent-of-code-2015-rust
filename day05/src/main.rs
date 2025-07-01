use aoc_lib::read_multiple_strings;
use fancy_regex::Regex;
use std::sync::LazyLock;

fn main() {
    let lines_result = read_multiple_strings(5);

    // Use `if let Ok(...)` for better error handling than `if .is_ok() .unwrap()`
    if let Ok(owned_lines_vec) = lines_result {
        let input_line_slices: Vec<&str> = owned_lines_vec
            .iter()
            .map(|s| s.as_str()) // Convert `&String` to `&str`
            .collect(); // Collect these `&str` into a new `Vec<&str>`

        println!("Day 5 Part 1: {}", solve_part1(&input_line_slices));
        println!("Day 5 Part 2: {}", solve_part2(&input_line_slices));
    } else {
        eprintln!("Error reading input: {:?}", lines_result.err());
    }
}

// These guys are expensive, so let's calculate them only once:
static THREE_VOWELS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap());
static FORBIDDEN_COMBINATIONS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"ab|cd|pq|xy").unwrap());
static TWICE_IN_A_ROW_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.)\1").unwrap());

fn is_nice(haystack: &str) -> bool {
    if FORBIDDEN_COMBINATIONS_RE.is_match(haystack).unwrap() {
        return false;
    }
    if THREE_VOWELS_RE.is_match(haystack).unwrap() == false {
        return false;
    }
    if TWICE_IN_A_ROW_RE.is_match(haystack).unwrap() == false {
        return false;
    }
    true
}

static DOUBLE_PAIRS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(..).*\1").unwrap());
static SEPARATED_PAIR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.).\1").unwrap());

fn is_very_nice(haystack: &str) -> bool {
    if DOUBLE_PAIRS_RE.is_match(haystack).unwrap() == false {
        return false;
    }
    if SEPARATED_PAIR.is_match(haystack).unwrap() == false {
        return false;
    }
    true
}

pub fn solve_part1(input: &[&str]) -> i32 {
    input
        .iter()
        .filter(|&x| is_nice(x))
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_part2(input: &[&str]) -> i32 {
    input
        .iter()
        .filter(|&x| is_very_nice(x))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

    #[test]
    fn test_is_nice_example_a() {
        assert_eq!(true, is_nice("ugknbfddgicrmopn"));
    }

    #[test]
    fn test_is_nice_example_b() {
        assert_eq!(true, is_nice("aaa"));
    }

    #[test]
    fn test_is_nice_example_c() {
        assert_eq!(false, is_nice("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_is_nice_example_d() {
        assert_eq!(false, is_nice("haegwjzuvuyypxyu"));
    }

    #[test]
    fn test_is_nice_example_e() {
        assert_eq!(false, is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_very_nice_example_a() {
        assert_eq!(true, is_very_nice("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn test_is_very_nice_example_b() {
        assert_eq!(true, is_very_nice("xxyxx"));
    }

    #[test]
    fn test_is_very_nice_example_c() {
        assert_eq!(false, is_very_nice("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_is_very_nice_example_d() {
        assert_eq!(false, is_very_nice("ieodomkazucvgmuy"));
    }
}
