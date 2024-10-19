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

/// Extracts the first and last digit (numeric or spelled-out) from a given line.
fn extract_first_last_digit(line: &str, digit_map: &HashMap<&str, i32>) -> Option<(i32, i32)> {
    let mut first_digit: Option<i32> = None;
    let mut last_digit: Option<i32> = None;
    let mut current_word = String::new();

    for c in line.chars() {
        if !current_word.is_empty() {
            if let Some(&digit) = digit_map.get(current_word.as_str()) {
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        }

        }
        else if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as i32;
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        } else if c.is_alphabetic() {
            current_word.push(c);
        } else {
            // Check for a complete spelled-out digit when encountering a non-alphabetic character
            if let Some(&digit) = digit_map.get(current_word.as_str()) {
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                last_digit = Some(digit);
            }
            current_word.clear(); // Clear the current word for the next sequence
        }
    }

    // Final check for the last word in case the line ends with letters
    if !current_word.is_empty() {
        if let Some(&digit) = digit_map.get(current_word.as_str()) {
            if first_digit.is_none() {
                first_digit = Some(digit);
            }
            last_digit = Some(digit);
        }
    }

    // Return the first and last digits if both are found
    if let (Some(first), Some(last)) = (first_digit, last_digit) {
        println!("{},{}",first, last);
        Some((first, last))
    } else {
        None
    }
}

/// Calculates the total sum of calibration values from the provided lines.
fn total_calibration_values(lines: Vec<String>) -> i32 {
    let digit_map = build_digit_map();
    lines.iter()
        .filter_map(|line| extract_first_last_digit(line, &digit_map))
        .map(|(first, last)| first * 10 + last)
        .sum()
}

/// Main function to execute the program logic.
pub fn execute() -> io::Result<()> {
    let file_path = "input_day1_part2.txt"; // Update with your input file path
    let lines = read_lines(file_path)?;

    let result = total_calibration_values(lines);
    println!("Total sum of calibration values: {}", result);

    Ok(())
}
