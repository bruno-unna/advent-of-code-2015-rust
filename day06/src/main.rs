use aoc_lib::read_multiple_strings;

use std::sync::LazyLock;
use std::sync::Mutex;

use regex::Regex;

fn main() {
    let lines_result = read_multiple_strings(6);

    // Use `if let Ok(...)` for better error handling than `if .is_ok() .unwrap()`
    if let Ok(owned_lines_vec) = lines_result {
        let input_line_slices: Vec<&str> = owned_lines_vec
            .iter()
            .map(|s| s.as_str()) // Convert `&String` to `&str`
            .collect(); // Collect these `&str` into a new `Vec<&str>`

        println!("Day 6 Part 1: {}", solve_part1(&input_line_slices));
        println!("Day 6 Part 2: {}", solve_part2(&input_line_slices));
    } else {
        eprintln!("Error reading input: {:?}", lines_result.err());
    }
}

const ROWS: usize = 1000;
const COLS: usize = 1000;

static FIELD: LazyLock<Mutex<Vec<Vec<i16>>>> =
    LazyLock::new(|| Mutex::new(vec![vec![0; COLS]; ROWS]));

#[derive(Debug)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Instruction {
    cmd: Command,
    corners: (Point, Point),
}

static CMD_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap()
});

impl Instruction {
    fn new(instruction_string: &str) -> Result<Instruction, String> {
        if let Some(captures) = CMD_RE.captures(instruction_string) {
            let cmd = match &captures[1] {
                "turn on" => Command::TurnOn,
                "turn off" => Command::TurnOff,
                "toggle" => Command::Toggle,
                _ => {
                    return Err(format!(
                        "unrecognized command in regex capture: '{}'",
                        &captures[1]
                    ));
                }
            };
            if let (Ok(x0), Ok(y0), Ok(x1), Ok(y1)) = (
                captures[2].parse::<i16>(),
                captures[3].parse::<i16>(),
                captures[4].parse::<i16>(),
                captures[5].parse::<i16>(),
            ) {
                return Ok(Instruction {
                    cmd, // Shorthand for cmd: cmd
                    corners: (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }),
                });
            } else {
                return Err(format!(
                    "couldn't parse coordinates at line\n\t{}",
                    instruction_string
                ));
            }
        } else {
            return Err(format!(
                "line\n\t{}\n couldn't be parsed as an instruction",
                instruction_string
            ));
        }
    }

    fn execute(&self) {
        println!(
            "executing instruction {:?} in range {:?} through {:?}",
            self.cmd, self.corners.0, self.corners.1
        );

        let mut field_guard = FIELD.lock().unwrap(); // This acquires a mutable reference to the Vec<Vec<bool>>

        for row in self.corners.0.y..=self.corners.1.y {
            for col in self.corners.0.x..=self.corners.1.x {
                let row_idx = row as usize;
                let col_idx = col as usize;

                match &self.cmd {
                    Command::TurnOn => field_guard[row_idx][col_idx] = 1,
                    Command::TurnOff => field_guard[row_idx][col_idx] = 0,
                    Command::Toggle => {
                        field_guard[row_idx][col_idx] = match field_guard[row_idx][col_idx] {
                            0 => 1,
                            _ => 0,
                        }
                    }
                }
            }
        }
    }
}

fn decode_instructions(input: &[&str]) -> impl Iterator<Item = Instruction> {
    input
        .iter()
        .map(|&s| Instruction::new(s))
        .map(|x| x.unwrap())
}

pub fn solve_part1(input: &[&str]) -> i32 {
    let instructions = decode_instructions(input);
    instructions.for_each(|instr| instr.execute());

    // Acquire the lock on the Mutex to get access to the inner Vec<Vec<bool>>
    let field_guard = FIELD.lock().unwrap(); // This yields a MutexGuard<Vec<Vec<bool>>>

    field_guard
        .iter()
        .map(|row| row.iter().filter(|&&x| x > 0).count())
        .sum::<usize>()
        .try_into()
        .unwrap()
}

pub fn solve_part2(_input: &[&str]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*; // Bring everything from outer scope into tests module

    #[test]
    fn test_part1() {
        let input = [
            "turn on 0,0 through 999,999",      // 1000000 lights on
            "toggle 0,0 through 999,0",         // 999000 lights on
            "turn off 499,499 through 500,500", // 998996 lights on
        ];

        assert_eq!(998_996, solve_part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = [
            "turn on 0,0 through 0,0",    // brightness is 1
            "toggle 0,0 through 999,999", // brighness is 2000001
        ];

        assert_eq!(2000001, solve_part2(&input));
    }
}
