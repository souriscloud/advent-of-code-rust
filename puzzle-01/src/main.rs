use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_calibration_value(line: &str) -> i32 {
    let mut first: i32 = 0;
    let mut last: i32 = 0;
    for c in line.chars() {
        if c.is_numeric() {
            if first == 0 {
                first = c.to_digit(10).unwrap() as i32;
            }
            last = c.to_digit(10).unwrap() as i32;
        }
    }

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

fn main() {
    println!("Advent of Code 2023 - Puzzle 01 [Part 1]");

    let file = File::open("./puzzle-01/input.txt")
        .expect("Something went wrong opening the file with the input! Expected path is: ./puzzle-01/input.txt");
    let reader = BufReader::new(file);
    let mut calibration_sum: i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.

        let calibration_val = get_calibration_value(&line);
        calibration_sum += calibration_val;

        println!("Line {}: {}: CAL: {}", index + 1, line, calibration_val);
    }

    println!("Calibration sum: {}", calibration_sum);
    println!("[End of Part 1]");

    println!("End of program.");
}
