use aoc_lib::read_multiple_strings;
use fancy_regex::Regex;
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

fn is_nice(haystack: &str) -> bool {
    // Compile regexes once, ideally outside the function if called repeatedly.
    // For this example, keep them here for illustration.
    let three_vowels_re = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let twice_in_a_row_re = Regex::new(r"(.)\1").unwrap();
    let forbidden_combinations_re = Regex::new(r"ab|cd|pq|xy").unwrap();

    if forbidden_combinations_re.is_match(haystack).unwrap() {
        return false;
    }
    if three_vowels_re.is_match(haystack).unwrap() == false {
        return false;
    }
    if twice_in_a_row_re.is_match(haystack).unwrap() == false {
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
    0
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
}
