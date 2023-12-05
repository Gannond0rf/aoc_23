use std::fmt::Display;

fn main() {
    println!("Advent of Code 2023");

    let input = include_str!("day1_input.txt");

    let day_1_pt1 = match day_1_pt1(input) { Ok(answer) => answer.to_string(), Err(err) => err.to_string() };
    let day_1_pt2 = match day_1_pt2(input) { Ok(answer) => answer.to_string(), Err(err) => err.to_string() };

    println!("Day 1 (Part 1): {day_1_pt1}");
    println!("Day 1 (Part 2): {day_1_pt2}");
}

fn day_1_pt1(input: &str) -> Result<i64, Error> {
    let numbers = input.lines().map(|line| {
        let digits: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
        if digits.is_empty() {
            Err(Error::InsufficientDigits)
        } else {
            str::parse(&format!("{}{}", digits[0], digits[digits.len() - 1]))
                .map_err(|err| Error::Other(Box::new(err)))
        }
    }).collect::<Result<Vec<i64>, Error>>()?;

    Ok(numbers.iter().sum())
}

fn day_1_pt2(input: &str) -> Result<i64, Error> {
    const NUM_TOKENS: [(&str, &str); 20] = [
        ("0", "0"),
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("six", "6"),
        ("four", "4"),
        ("zero", "0"),
        ("five", "5"),
        ("nine", "9"),
        ("three", "3"),
        ("seven", "7"),
        ("eight", "8"),
    ];

    let numbers = input.lines().map(|line| {
        let mut digits = Vec::new();
        for start in 0..line.len() {
            for (token, digit) in NUM_TOKENS {
                if (start + token.len()) > line.len() { continue };
                let slice = &line[start..(start + token.len())];
                if slice == token {
                    digits.push(digit);
                    break;
                }
            }
        }
        if digits.is_empty() {
            Err(Error::InsufficientDigits)
        } else {
            str::parse(&format!("{}{}", digits[0], digits[digits.len() - 1]))
                .map_err(|err| Error::Other(Box::new(err)))
        }
    }).collect::<Result<Vec<i64>, Error>>()?;

    Ok(numbers.iter().sum())
}

#[derive(Debug)]
pub enum Error {
    Other(Box<dyn std::error::Error>),
    InsufficientDigits,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(err) => write!(f, "{err}"),
            Error::InsufficientDigits => write!(f, "Insufficient digits"),
        }
    }
}
