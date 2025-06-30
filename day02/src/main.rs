use aoc_lib::read_multiple_strings;

fn main() {
    let lines_result = read_multiple_strings(2);

    // Use `if let Ok(...)` for better error handling than `if .is_ok() .unwrap()`
    if let Ok(owned_lines_vec) = lines_result {
        // `owned_lines_vec` now owns the Vec<String> data.

        // Create a Vec<&str> where each &str borrows from the Strings inside `owned_lines_vec`.
        let input_line_slices: Vec<&str> = owned_lines_vec
            .iter()
            .map(|s| s.as_str()) // Convert `&String` to `&str`
            .collect(); // Collect these `&str` into a new `Vec<&str>`

        // Now, `input_line_slices` is a `Vec<&str>` that borrows from `owned_lines_vec`.
        // We pass a reference (slice) to it, so it's not moved.
        println!("Day 2 Part 1: {}", solve_part1(&input_line_slices));
        println!("Day 2 Part 2: {}", solve_part2(&input_line_slices));
    } else {
        eprintln!("Error reading input for Day 2: {:?}", lines_result.err());
    }
}
fn string_to_tuple(input: &str) -> Option<(i32, i32, i32)> {
    let mut iterator = input.split('x');

    let w: i32 = iterator.next()?.parse().ok()?;
    let h: i32 = iterator.next()?.parse().ok()?;
    let l: i32 = iterator.next()?.parse().ok()?;

    Some((w, h, l))
}

fn calculate_paper((w, l, h): (i32, i32, i32)) -> i32 {
    let areas = [l * w, w * h, h * l];
    let min_area = areas.iter().min().unwrap();
    let areas_sum: i32 = areas.iter().map(|&x| x).sum();
    2 * areas_sum + min_area
}

pub fn solve_part1(input: &[&str]) -> i32 {
    let input_tuples = input
        .iter()
        .map(|as_string: &&str| string_to_tuple(as_string).unwrap());
    let dimensions = input_tuples.map(|t| calculate_paper(t));
    dimensions.sum()
}

pub fn solve_part2(input: &[&str]) -> i32 {
    0
}

// --- Unit Tests Section ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1_example_a() {
        let example_input = "2x3x4";
        let converted_input: Vec<&str> = Vec::from([example_input]);
        assert_eq!(58, solve_part1(&converted_input));
    }

    #[test]
    fn test_solve_part1_example_b() {
        let example_input = "1x1x10";
        let converted_input: Vec<&str> = Vec::from([example_input]);
        assert_eq!(43, solve_part1(&converted_input));
    }
}
