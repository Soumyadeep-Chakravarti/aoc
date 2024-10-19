use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads lines from a specified file and returns them as a vector of strings.
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}


/// Maps spelled-out numbers to their corresponding digits.
fn build_digit_map() -> HashMap<&'static str, i32> {
    let mut digit_map = HashMap::new();
    digit_map.insert("zero", 0);
    digit_map.insert("one", 1);
    digit_map.insert("two", 2);
    digit_map.insert("three", 3);
    digit_map.insert("four", 4);
    digit_map.insert("five", 5);
    digit_map.insert("six", 6);
    digit_map.insert("seven", 7);
    digit_map.insert("eight", 8);
    digit_map.insert("nine", 9);
    digit_map
}

/// Calculates the total sum of calibration values from the provided lines.
fn total_calibration_values(lines: Vec<String>) -> i32 {
    let mut total_sum = 0;

    for line in lines {
        let mut first = -1; // Sentinel value for the first digit
        let mut last = -1;  // Sentinel value for the last digit

        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                if first == -1 {
                    first = digit as i32; // Set first digit
                }
                last = digit as i32; // Update last digit
            }
        }

        if first != -1 && last != -1 {
            total_sum += first * 10 + last;
        }
    }

    total_sum
}

pub fn execute() -> io::Result<()> {
    let file_path = "input_day1_part1.txt"; // Update with your input file path
    let lines = read_lines(file_path)?;

    let result = total_calibration_values(lines);
    println!("Total sum of calibration values: {}", result);

    Ok(())
}
